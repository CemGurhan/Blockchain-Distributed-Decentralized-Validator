# Repo for Non-Distributed Centralized (NDC) IID/Non-IID training tests

Makefile targets explained:

 * To run a test with four validators in a Non-IID scenario, execute command `make four` in the backend folder. You will also have to run the relevant command in the lightclient application locally for the training rounds to begin.
 
 * To run a test with four validators in an IID scenario, execute command `make n-four` in the backend folder. You will also have to run the relevant command in the lightclient application locally for the training rounds to begin. 

 * To run a test with a given sync-policy, simply add your sync policy to the end of your makefile target: `make <IID vs Non-IID taget>-<Desired sync policy>`. E.g to run a Non-IID scenario with SSP run `make four-SSP` and for IID run `make n-four-SSP`.

## Different executable sync policies:

### Bulk Synchronous Parallel (BSP)
Abbreviated as BSP, it is known to be the simplest approach to preserve consistency
by alternating between rounds of computation followed by
communication. The main pro of this model is that it has
the best consistency (hence, fastest convergence), yet its main
bottleneck is that finished nodes have to wait for all worker
nodes to finish their computation at each synchronization
barrier. In our variant of BSP, each training round is
limited by a fixed period that ends with a strict deadline after
which no further trainer updates are considered for that round.

### Stale Synchronous Parallel (SSP)
This model is similar to BSP but with more relaxed synchronization barriers that allow
a certain number of iterations beyond the preset number. After
that leeway is reached, all workers are paused. Given its
nature as a compromise to BSP, it has good convergence insur-
ances in low to moderate staleness, beyond which convergence
rates start to decay. In our design, SSP is modelled as BSP
with the possibility of a deadline extension which is a fraction
of the original round period determined in proportion to the
ratio of trainers that have not yet finished training for that
round, denoted by slack ratio. Meanwhile, trainers that already
finished by the deadline are allowed to continue training for
at most N more steps before the deadline extension finishes.

### Barrierless Asynchronous Parallel (BAP)
 This model
represents the opposite end of the spectrum to BSP as it almost
totally eliminates the cost of synchronization by allowing
waitless, asynchronous communication between nodes, which
achieves a very low overhead (hence, higher speedups), but it
suffers from potentially slow and even incorrect convergence
with increased delays. In our variant of BAP, trainers are
allowed to train for as long as a round may extend; that is,
until a new model is released, trainers can keep advancing
local training, periodically sharing it with a validator. A new
model is released when a minimum ratio of labour has been
submitted, and that’s when trainers should pull in the new
model.

### Approximate Synchronous Parallel (ASP)
 Although our system does not support ASP, the scheme remains note-
worthy. Unlike the SSP model, ASP is concerned with limiting
parameter accuracy instead of staleness. It does so by ignoring
updates that are not significant to spare the synchronization
costs of them. However, one downside of that approach is
that it is not easy to decide which parameters are insignificant,
along with increased complexity of implementation.

