# subscription

This is fastn loadable wasm file source, that helps you add email subscription to your site. This
uses `fastn_user` table to store subscription data, and uses a cookie named `fastn_tid` to keep
track of the current user.

## Actions

This wasm file exposes the following routes:

### Subscribe: /subscribe/

This route accepts optional name, optional phone and optional email address. Email address 
or phone is required if the user is not already logged in.

This route also takes `source` as optional argument. If this is present we add it to 
`data -> subscription -> source` set (we do not add duplicates here).

We add `data -> subscription -> subscribed` as `true` when the user subscribes. This is
set to `false` by the `/unsubscribe/` route. Any non-transactional mail should only be sent
if this is set to `true`.

This route also accepts optional `topic`, which is added to `data -> subscription -> topics`
set (we do not add duplicates here).

If there is no user in the `fastn_user` table, this route creates one.

This view takes an optional `next`, with default value of `/thank-you/`, and user is redirected
to this URL after the subscription is successful.

### Unsubscribe: /unsubscribe/

This takes optional `topic`. If `topic` is not passed we set `data -> subscription -> subscribed`
to `false` (they no longer receive any non-transactional mail from us).

This view takes an optional `next`, with default value of `/goodbye/`, and user is redirected
to this URL after the unsubscription is successful.

This action also empties the `data -> subscription -> topics` set.

### Set Tracker: `/set-tracker/?t=<encrypted/hashed uid>`

Set a tracker cookie.

The `t` is an encrypted string which contains the user id (as i64). If `t` is
not provided, current logged in user id is used. If there is no logged in user,
set an empty tracker cookie.

The tracker cookie should be also set:
- on subscribe - set tracker id cookie
- on log in - set tracker id cookie (in email-auth.wasm)
- on cookie consent - set tracker id cookie

## Data

This wasm file also exposes this data fetch URL:

### /topics/

This returns a list of subscriptions (`topics`) for the current user. It will return:

```json
{
  "subscribed": true,
  "topics": ["list", "of", "topics"]
}
```

### /is-subscribed/?topic=<>

This returns boolean indicating if the current user is subscribed to the given topic. This can be used to
show subscription dialog. Though it is recommended that you use the corresponding query.

## Queries

ftd files are recommended to use the following queries.

### Find all lists the current user is subscribed to.

This query can be used to create a page that shows the list of subscription of a user.
The `/subscribe/` and `/unsubscribe/` actions can be used to modify this list.

```ftd
-- integer uid:
;; this processor looks for user via tracker id, this is only good for email
;; subscription scenarios
$processor$: pr.tracked-user-id 

-- string list topics:
$processr$: pr.sql-query
id: $uid

SELECT 
    data -> subscription -> topics
FROM 
    fastn_user
WHERE 
    id = $1  -- since id can be zero, this wont match anyone if user is not logged in
```

### See if the user is subscribed to a topic

```ftd
-- integer uid:
$processor$: pr.tracked-user-id

-- boolean subscribed:
$processor$: pr.sql-query
id: $uid
topic: <some topic>

SELECT EXISTS (
    SELECT 1
    FROM json_each(data -> 'subscription' -> 'topics')
    WHERE value = $topic
)
from fastn_user where id = $id;
```

### See if the user is subscribed to anything

```ftd
-- integer uid:
$processor$: pr.tracked-user-id

-- boolean subscribed:
$processor$: pr.sql-query
id: $uid
topic: <some topic>

SELECT data -> 'subscription' -> subscribed
from fastn_user where id = $id;
```


## FAQ

### Can a user be subscribed without any topic?

For most websites there would be no topics, or rather a "default" topic. So instead of adding
"default" to all topics, and checking if this "default" exists in topics, we keep a separate
subscribed boolean (which is always subscribed to "default" topic).
