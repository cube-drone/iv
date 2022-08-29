# Shard/Region

These regions might be "us-1", "us-2", "jp-1", "eu-1"

Okay, here's how I think that iv shards are going to work:

 * Each server maintains its own auth database and groups
 * Every group is tied to a user
 * Every user logs in with a portable JWT
 * There's a master-redis that just contains:
	 * Auth-token revocations
	 * User => Region mappings
	 * Mods & Bans


### How do we log in? 
If you go to any shard and log in, you'll be TEMPORARILY REDIRECTED to the nearest shard after entering your email address.

If you go to the registration form, you'll be automatically redirected to the nearest-ish shard, although you'll be able to choose your shard. 

After successfully logging in, you are granted an auth JWT which is portable across regions. 

#### How do we know you're online?
We don't. We only know whether or not users are online within groups, which each are their own full-fledged chat environments.

#### How do we connect?

The client starts in offline mode, attached to your default, private, personal group.

You can log in with an Anon account, allowing you to join public+anon groups and watch them in read-only mode.

The client is capable of keeping track of multiple sessions and identities, although not at the same time. (protect an identity with a pin?)

You can (log in with your/create a) personal account, allowing you to log in. 