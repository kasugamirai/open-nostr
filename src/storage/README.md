# Storage

## Relay Sets

A relay set is a set of relay URIs. Unlike most nostr apps, Capybastr uses relay set to quickly apply a set of relay (though a preheated client) for query & sub.

1. Relay Set (K - V):
   name - (a set of relays)

2. Relay Set Names (K - V):
   "RELAY_SET_NAMES" - (a list of unique relay_set_name)

## Custom Sub

A custom sub is a combination of filters with additional data. The struct for UI is defined in state/subscription.rs.

1. Cunstom Sub (K - V):
    name - {relay_set: Relay Set, filters: [FilterTemp]}
    "CUSTOM_SUBS" - (a list of unique custom_sub_name)

### things to do
1. Define relay sets and custom sub
2. Alter state/subscriptions to use struct defined in storage
3. Use indexdb to store and read those structs