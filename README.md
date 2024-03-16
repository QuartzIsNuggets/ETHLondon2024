# Travel route enforcement framework

## Why?

TODO

## Terms

The *end service* is the service using our framework.

The *Trajectory Enforcer* is a smart contract deployed on Arbitrum (Stylus) that can
create contracts binding the following two actors.

The *route issuer* wants a travel to be done following a given path. It pays for the
filling of the route, for the gas and for the Oracle.

The *route filler* wants to be payed to complete a travel following a path. It updates
the Trajectory Enforcer on a regular basis.

## Workflows

The workflow depends on whether the contract deems it is breached or not. The first
steps are common.

1. Within some application, a route is desired by a route issuer.
2. Using a software provided by the framework, data needed to enforce the route is
   derived from the exact route.
3. The route issuer issues a contract on the Trajectory Enforcer, pre-paying for
   however much the end service would like and the expected gas usage. The contract
   holds how much the collateral is expected to be.
4. Using the external service, a route filler is selected/elected/etc. It activates
   the contract instance by paying a collateral to cover an upper bound of gas fees.
5. A software provided by the framework observes the advancement of the contract.

### Honest contract filling

6. The route filler sends the expected updates to the contract at the expected times.
7. The contract detects the arrival. It pays for the filling and the gas to the
   filler's address.
8. The contract entry is now deleted from the Trajectory Enforcer.

### Dishonest contract fillings

##### Updating the contract with bad positions

6. The route filler gets off-road too much or doesn't move.
7. The contract detects the issue, and sets itself in alert mode. It is locked.
8. The software run by the issuer informs the end service.
9. The end service has control over the contract and arbitrates.

##### Not updating the contract

6. The route filler fails to send updates on time.
7. The software run by the issuer detects the issue. It informs the end service and
   locks the contract.
8. The end service has control over the contract and arbitrates.
