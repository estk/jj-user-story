# Jujutsu onboarding for feature branch teams

## Tips for atomic commits

When any of these situations arise consider how it will be commited:

- Refactor this file into a multi-file module
- Renames (f2)
- Consider breaking features into parts and incrementally implementing them

Most of all, read the code, come up with a plan for not just the impl but the commits to impl it.

## Example

### Development

You have an `EventDispatcher` which spawns a worker to process and dispatch events.
The feature request is to add a configuration value which allows for certain events to be replayed at an interval.

```sh
jj new main -m 'Event replay'
jj branch create <me>/<issue_number>-event_replay
```

```text
@  zsstsonw esims89@gmail.com 2024-03-10 15:08:46.000 -07:00 me/0-event_replay 7296cf36
│  (empty) Event replay
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

#### Tip

If you find you have a lot of clutter in your log, consider using the following config:

```toml
[revset-aliases]
"wc_trunk()" = "(trunk..@):: | (trunk..@)-"
trunk = "latest((present(main) | present(master) | present(devel)) & remote_branches())"
...
[aliases]
lt = ["log", "-r", "wc_trunk()"]
```

- You realize that `EventDispatcher` will need some refactoring.
- Split `EventDispatcher` components into separate files.

⚫ Revision (`zsstsonw`): `jj commit -m 'Move EventDispatcher supporting components out to separate files'`

```sh
jj log
@  mmlqunzx esims89@gmail.com 2024-03-10 15:19:25.000 -07:00 1e9c0c5e
│  (empty) (no description set)
◉  zsstsonw esims89@gmail.com 2024-03-10 15:19:25.000 -07:00 me/0-event_replay c9aa16a3
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

This updates the commit message and creates a new change in the working copy.

- Start work on feature
- Add a field to the worker state which is the interval at which the event will be replayed.
- Add the logic to capture a matching event.
- Add logic to replay said event on said interval.

⚫ Revision (`swloqlsw`): `jj commit -m 'Add event replay logic and configuration'`

```sh
jj log
@  yxvzuomu esims89@gmail.com 2024-03-10 15:34:43.000 -07:00 c8361530
│  (empty) (no description set)
◉  swloqlsw esims89@gmail.com 2024-03-10 15:34:43.000 -07:00 6d3da21d
│  Add event replay logic and configuration
◉  zsstsonw esims89@gmail.com 2024-03-10 15:21:34.000 -07:00 me/0-event_replay 113c1c2d
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

- Write the unit-test to verify event is replayed.
- Tests fail.
- Realize that there are two sink's which events must be sent to.
- Realize the bug would have been prevented by having a single method which dispatches events.

⚫ Revision (`yxvzuomu`): `jj commit -m 'Event replay unit tests'`
```sh
jj log
@  okuzlqvo esims89@gmail.com 2024-03-10 15:48:44.000 -07:00 d7d0248c
│  (empty) (no description set)
◉  yxvzuomu esims89@gmail.com 2024-03-10 15:48:44.000 -07:00 b82ad2b5
│  Event replay unit tests
◉  swloqlsw esims89@gmail.com 2024-03-10 15:34:43.000 -07:00 6d3da21d
│  Add event replay logic and configuration
◉  zsstsonw esims89@gmail.com 2024-03-10 15:21:34.000 -07:00 me/0-event_replay 113c1c2d
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

- Realize the commit would be most easily reviewed just after revision `zs`
- Realize you need to add a commit between `zs` and `sw`

⚫ Revision (`aa`): `jj new -A a -m "EventDispatcher single dispatch point"`

```sh
jj log
◉  okuzlqvo esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 5ff75a63
│  (no description set)
◉  yxvzuomu esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 e718e888
│  Event replay unit tests
◉  swloqlsw esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 b8940058
│  Add event replay logic and configuration
@  roztqptz esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 6076e63b
│  (empty) EventDispatcher single dispatch point
◉  zsstsonw esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 me/0-event_replay f7fe1340
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

- Perform the refactor
- View the log of descendent commits `jj log -r '@::'`
- Realize there is a conflict at `b`

To fix the conflicts you could just `jj edit c` and fix conflicts directly. A better way would be creating a separate revision to perform the resolution in.

⚫ Revision (`ba`): `jj new -A ba -m "Resolve conflict"`
```sh
jj log
...
◉  uqkrlswz esims89@gmail.com 2024-03-10 16:07:01.000 -07:00 15156fe1 conflict
│  Resolve conflict
◉  swloqlsw esims89@gmail.com 2024-03-10 16:07:01.000 -07:00 d2692545 conflict
│  Add event replay logic and configuration
◉  roztqptz esims89@gmail.com 2024-03-10 16:07:01.000 -07:00 3a3cdc97
│  EventDispatcher single dispatch point
◉  zsstsonw esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 me/0-event_replay f7fe1340
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

- Choice 1: `jj resolve` which should start the merge tool
- Choice 2: Manual
  - `jj restore --from aa`
  - `jj workspace add` to create a new workspace
  - In the new workspace `jj edit aa`
  - Manually copy changes from new workspace to `ba`
- `jj squash` to abandon `ba` by moving resolution into `b`

⚫ Revision (`d`): `jj new c`

```sh
◉  yxvzuomu esims89@gmail.com 2024-03-10 16:08:09.000 -07:00 cd13c92f
│  Event replay unit tests
◉  swloqlsw esims89@gmail.com 2024-03-10 16:08:09.000 -07:00 540aa17e
│  Add event replay logic and configuration
◉  roztqptz esims89@gmail.com 2024-03-10 16:07:01.000 -07:00 3a3cdc97
│  EventDispatcher single dispatch point
◉  zsstsonw esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 me/0-event_replay f7fe1340
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

- Verify unit tests now pass

```sh
jj log
@  okuzlqvo esims89@gmail.com 2024-03-10 16:11:22.000 -07:00 271e7035
│  (no description set)
◉  yxvzuomu esims89@gmail.com 2024-03-10 16:11:22.000 -07:00 45f0a1fb
│  Event replay unit tests
◉  swloqlsw esims89@gmail.com 2024-03-10 16:11:22.000 -07:00 7d71135d
│  Add event replay logic and configuration
◉  roztqptz esims89@gmail.com 2024-03-10 16:07:01.000 -07:00 3a3cdc97
│  EventDispatcher single dispatch point
◉  zsstsonw esims89@gmail.com 2024-03-10 15:52:39.000 -07:00 me/0-event_replay f7fe1340
│  Move EventDispatcher supporting components out to separate files
◉  sqntlpvk esims89@gmail.com 2024-03-10 15:05:27.000 -07:00 main 00e234f8
│  initial commit
```

### Commit cleanup and pushing

We now have the above revset with two refactors, logic addition and test addition.
Those revs are exactly the units that we would like our team to review. Our choices are:

1. Push as one PR and let them consider the commits
2. Push an empty branch from main and get PR's reviewed into that branch (stacked pr's)
3. Push each rev as a separate branch directly merging to main, subsequent revs get rebased.

Before making a decision I think it's important to zoom back out and consider the goals of a review
process, and the practical considerations of merge order.
