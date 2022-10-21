CD into the backend folder, and open a terminal. Build your backend out with 2 validators by typing in: bash build_finalize.sh -n 2 -b -c -j

After build is finished, I'd recommend splitting your terminal to view logs at the same time more easily. 

On your first terminal, in order to run your first validator (leading validator with numerical identifier 0) enter: bash run_node.sh  0 BAP 1 1 MNIST28X28

Your main.rs is ran using your exonum_run command that's executed inside the run_node.sh script. This will then run different parts of the patches folder in this order: 

exonum-cli src/lib.rs run() -> exonum-node/src/lib.rs initialize() -> exonum-node/src/lib.rs run() -> exonum-node/src/events/mod.rs run() -> exonum_node/src/events/network.rs async run() -> exonum-api/src/manager.rs start_servers() (this final one will start servers on the public and private ports for the node)

On your second terminal, run your second validator (validator with numerical identifier 1) by entering: bash run_node.sh  1 BAP 1 1 MNIST28X28

This will run via the same logic that is ran once you start the first validator

Now, your first node will attempt to make a connection to a potential peer node. This is done in exonum_node/src/events network.rs 
create_connection(). The current node seems to attain the public key of the other node from the example folder, where the public keys of each node are stored. This seems to happen twice on the current node, and three times on the second node. 

Both nodes then, at the same time as the previous connection attempts, establish a handshake with each other via exonume-node/src/events/noise/wrappers/sodium_wrapper/handshake.rs send().

After this, the nodes continiously listen for and recieve connection messages from each other. Every time a connection message is recieved, exonum_node/src/basic.rs handle_connect() is ran through. As the Received Connect message has the same public address as our external_address, only that is registered. This means that the node that we recieved the message from has the same public address as us, which can be seen from self.state.our_connect_message().payload().host (which is the our nodes address, where .host denotes the address) being equal to the other nodes address (message.payload().host.clone()). 

At the same time, the lightclient has been training on it's locally cached model. It eventually sends this trained model to the backend via:

await exonum.send(explorerPath, serialized, 1000, 3000)
    .then((obj) => {console.log(obj); })
    .catch((obj) => { console.log(obj); clearMetadataFile()})
    .finally(() => { can_train = true; })

inside trainNewModel() from app.js, which is called in main() in app.js. NB model training occurs in trainNewModel() after it calls fetchPythonWeights() on line 63 (I know youll get OCD about where this is called Yunus, so there it is). 

Serialized in the exonum.send call is the share updates transaction serialized into bytes. ShareUpdates transaction is created in trainNewModel() on line 34 as such: 

const ShareUpdates = new exonum.Transaction({
        schema: proto.TxShareUpdates,
        serviceId: SERVICE_ID,
        methodId: SHAREUPDATES_ID,
    })

where SERVICE_ID is the service ID of the backend (this being 3) which is declared in the LC on line 31 in trainNewModel()). This number is instantiated in src/lib.rs at the very bottom with: 

impl DefaultInstance for MachineLearningService {
    const INSTANCE_ID: InstanceId = 3;
    const INSTANCE_NAME: &'static str = "ml_service";
}

SHAREUPDATES_ID is the id of the transaction service method share_updates (this being 0) which is declared in the LC on line 34 in trainNewModel(). This is instantiated in src/transaction.rs as such: 

#[exonum_interface]
pub trait MachineLearningInterface<Ctx> {
    /// Output returned by the interface methods.
    type Output;

    /// Proposes a model update
    #[interface_method(id = 0)]
    fn share_updates(&self, ctx: Ctx, arg: ShareUpdates) -> Self::Output;

    /// Signal a sync barrier
    #[interface_method(id = 1)]
    fn sync_barrier(&self, ctx: Ctx, arg: SyncBarrier) -> Self::Output;
}

After the light client sends the share_updates transaction to the backend (which contains the trained model), the backend achkowledges that is has recieved a TRANSACTION EVENT. Before any of the logic in transaction.rs can be called, the transaction must first be broadcast to all other validators, handled and the underlying model validated. 

The method exonum/src/blockchain/api_sender.rs  broadcast_transaction() seems to be called immedietely after the backend gets the message, which then calls exonum/src/blockchain/api_sender.rs send_message() (an async await function). 

After this, the transaction event is handled. This is done by exonum-node/src/event_impl.rs handle_event being called. Inside here, a match statement is ran through, and the fact that the event is of type transaction cause the transaction arm of the match statement to be executed. Inside this arm, the function exonum-node/src/consensus.rs handle_incoming_tx() is called. handle_incoming_tx() then calls exonum-node/src/consensus.rs handle_tx(). handle_tx() then begins validating the model.

After validating is done and is successful, the model is cached in handle_tx() on line 946. An ok() message is sent up from handle_tx() to handle_incoming_tx(). As the ok() message is recieved from handle_incoming_tx(), the method exonum-node/src/lib.rs broadcast() is called (This message is also called from elsewhere, whcih means it is constantly ran through. This can be evidenced by placing logs inside of it). This method then broadcasts the message to the second node. 

Whilst this has all been happening, the nodes have stopped sending connection messages to each other. It would appear connection messages are not sent during validation. 

The second node recieves a message, and exonum-node/src/basic.rs handle_message() is called. handle_message() seems to always be called, as the nodes are constantly communicating (except during validation). As the message sent to this node contains a transaction, the Message::Service(Service::AnyTx(msg)) arm of the match statement is ran through, which contains a call to exonum-node/src/consensus.rs handle_tx(). handle_tx() again validates the transaction, and sends an ok() back up to handle_message(). 

AFter this, the nodes continue sending connection messages to each other. It would appear the second node sends a rpool transaction equest message to the first node. This can be evidenced by the Message::Requests(ref msg) => self.handle_request(msg) arm being ran in handle_message(). This calls exonum-node/src/request.rs handle_request, which then runs the arm Requests::PoolTransactionsRequest(ref msg) => self.handle_request_pool_txs(msg) in the match statement. This calls exonum-node/src/request.rs handle_request_pool_txs(), which in turn calls exonum-node/src/request.rs send_transactions_by_hash(). This then pulls out the cached transaction by calling the function exonum/src/blockchain/mod.rs get_transaction(), which in turn calls another function called get_transaction() (which exists on line 97) in the same file.





break validate




