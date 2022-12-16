# Repo for Non-Distributed Centralized (NDC) IID/Non-IID training tests

 To run a test with four validators in a Non-IID scenario, execute command `make four` in the backend folder. To run a test with four validators in an IID scenario, execute command `make n-four` in the backend folder. You will also have to run the relevant command in the lightclient application locally for the training rounds to begin. 

## Different executable sync policies:

### Bulk Synchronous Parallel (BSP)
Abbreviated as BSP, it is known to be the simplest approach to preserve consistency
by alternating between rounds of computation followed by
communication. The main pro of this model is that it has
the best consistency (hence, fastest convergence), yet its main
bottleneck is that finished nodes have to wait for all worker
nodes to finish their computation at each synchronization
barrier [18]. In our variant of BSP, each training round is
limited by a fixed period that ends with a strict deadline after
which no further trainer updates are considered for that round.

### Stale Synchronous Parallel (SSP)
This model is similar to BSP but with more relaxed synchronization barriers that allow
a certain number of iterations beyond the preset number. After
that leeway is reached, all workers are paused [8]. Given its
nature as a compromise to BSP, it has good convergence insur-
ances in low to moderate staleness, beyond which convergence
rates start to decay [18]. In our design, SSP is modelled as BSP
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
with increased delays [18]. In our variant of BAP, trainers are
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
costs of them [18]. However, one downside of that approach is
that it is not easy to decide which parameters are insignificant,
along with increased complexity of implementation.

