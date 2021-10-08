# Rolling Slots Module
![RollingSlots](res/RollingSlots.png)

---

At this moment this concept is being implemented [here](https://github.com/paritytech/polkadot/pull/3943).

---

## Overview
The name of this repo does a horrible job descbribing the task this pallet is meant to do.
 But I thought it was a funny word play.

That said, this pallet aims to automate the lease allocation on a serie of given slots.
 So a more proper name would have been _rolling paras_ ? Anyway, the idea is to maintain
 two different groups of slots, long term and short term so the paras opting for those slots
 can receive leases on a rate defined depending on the type of the slot.

## To take into account

### Some early thoughts on how the pallet is expected to work.

- The pallet only manages onboardings and offboardings, parathreads should register as such like usual.
- LongTerm opting paras would be added by SUDO calls.
- ShortTerm opting paras must be added by the owner/manager account of the ParaId.
- LongTerm paras should not be rotated.
- ShortTerm paras have to be rotated between a number of defined slots with a defined rate.
- SUDO can define the number of slots, the rotation rates, and the number of
 leases paras will be granted.
- Rotate a number of paras means downgrade from parachain to parathread the
 actual onboarded paras and upgrade from parachain to parathread the next 
 #`ShortTermSlots` parachains from `ShortTermParas`. In a round robin way
- As for LongTerm paras there is not rotation intended, new leases will be added for them.

### Descriptive flow

- Registering a para for a LongTerm slot

Parathread should have been registered by owner/manager previously. Once that is done, a SUDO call to `register_long_term_para` can be made to include this `ParaId` into the LongTerm group.
After successful registration this parathread will be eventually upgraded to a parachain and receive `LongTermSlotDuration` leases until downgraded.

- Registering a para for a ShortTerm slot

Parathread should have been registered by owner/manager previously. Once up as a parathread the owner/manager can call this pallet's extrinsic `register_short_term_para` to include its `ParaId` into the ShortTerm group.
After successful registration this parathread will idle until the corresponding rotation upgrade it to parachain. It will remain upgraded during `ShortTermSlotDuration` leases after which it is going to be downgraded to a parthread again, remaining as such until its `ParaId` is rotated again.

- Deregistering paras

SUDO or owner/manager accounts should be able to deregister a `ParaId` for a group.

### Dependencies and useful resources

Use [Scheduler](https://github.com/paritytech/substrate/blob/master/frame/scheduler/src/lib.rs) to create and automate the task of rotation and lease providing.
To manage paras we can find the relevant logic at [runtime_parachain::paras](https://github.com/paritytech/polkadot/blob/master/runtime/parachains/src/paras.rs) lib and its two wrappers [paras_registrar](https://github.com/paritytech/polkadot/blob/master/runtime/common/src/paras_registrar.rs) and [paras_sudo_wrapper](https://github.com/paritytech/polkadot/blob/master/runtime/common/src/paras_sudo_wrapper.rs)are also good resources to see how `runtime_parachains::paras` is used.
[`ParaId`](https://github.com/paritytech/polkadot/blob/master/parachain/src/primitives.rs#L139) read from here.



License: Apache-2.0
