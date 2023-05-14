// Add yourself to the club if the size is less than two.
#[pallet::weight(10_000)]
pub fn add_self_to_club(origin: OriginFor<T>) -> DispatchResult {
    // Check if the sender is a signed account
    let sender = ensure_signed(origin)?;

    // Get the current club members
    let mut club_members = ClubMembers::<T>::get();

    // Check if the sender is already a member of the club
    if club_members.binary_search(&sender).is_ok() {
        return Err(Error::<T>::AlreadyMember.into());
    }

    // Check if the club has less than two members
    if club_members.len() >= 2 {
        return Err(Error::<T>::ClubFull.into());
    }

    // Add the sender to the club
    club_members.push(sender.clone());

    // Update the club members
    ClubMembers::<T>::put(&club_members);

    // Emit an event
    Self::deposit_event(Event::MemberAdded);

    Ok(())
}