# Rolling Slots Module

## Overview
This pallet is in charge of managing two groups of parachain slots. Long term slots and short term slots, which will be receiving leases with different cadence.
Also, short term slots are expected to be less than the paras opting for one of this slots, and so, this pallet also implements the logic to rotate parathreads between these short term slots. Enabling a mayor number of parathreads be upgraded into parachains even if it is for a reduced ammount of time.

License: Apache-2.0
