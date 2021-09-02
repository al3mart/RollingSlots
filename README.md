# Rolling Slots Module
![RollingSlots](res/RollingSlots.png)

---

## Overview
This pallet is in charge of managing two groups of parachain slots. Long term slots and short term slots, which will be receiving different ranges of leases.
Also, short term slots are expected to be less than the paras opting for one of these slots, and so, this pallet also implements the logic to rotate parathreads between the available short term slots. Enabling a mayor number of parathreads to be upgraded into parachains even if it is for a reduced period of time.

License: Apache-2.0
