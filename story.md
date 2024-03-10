# Jujutsu onboarding for feature branch teams

## Tips for atomic commits

When any of these situations arise consider how it will be commited:

- Refactor this file into a multi-file module
- Renames (f2)
- Consider breaking features into parts and incrementally implementing them

Most of all, read the code, come up with a plan for not just the impl but the commits to impl it.

## Example happyish path

You have an `EventDispatcher` which spawns a worker to process and dispatch events.
The feature request is to add a configuration value which allows for certain events to be replayed at an interval.

```sh
jj new main -m 'Event replay'
jj branch create <me>/<issue_number>-event_replay
```

- You realize that `EventDispatcher` will need some refactoring.
- Split `EventDispatcher` components into separate files.

⚫ Revision (`a`): `jj commit -m 'Move EventDispatcher supporting components out to separate files'`

This updates the commit message and creates a new change in the working copy.

- Start work on feature
- Add a field to the worker state which is the interval at which the event will be replayed.
- Add the logic to capture a matching event.
- Add logic to replay said event on said interval.

⚫ Revision (`b`): `jj commit -m 'Add event replay logic and configuration'`

- Write the unit-test to verify event is replayed.
- Tests fail.
- Realize that there are two sink's which events must be sent to.
- Realize the bug would have been prevented by having a single method which dispatches events.
- Realize the refactor will require ownership transfer of the processed-event-queue from the dispatcher to worker loop-state.

⚫ Revision (`c`): `jj commit -m 'Event replay unit tests'`

- Realize the commit would be most easily reviewed just after revision `a`
- Realize you need to add a commit between `a` and `b`

⚫ Revision (`aa`): `jj new -A a -m "Move EventDispatcher output queue ownership to worker State to enable single dispatch point"`

- Perform the refactor
- View the log of descendent commits `jj log -r '@::'`
- Realize there is a conflict at `b`

To fix the conflicts you could just `jj edit c` and fix conflicts directly. A better way would be creating a separate revision to perform the resolution in.

⚫ Revision (`ba`): `jj new -A ba -m "Resolve conflict"`

- Choice 1: `jj resolve` which should start the merge tool
- Choice 2: Manual
  - `jj restore --from aa`
  - `jj workspace add` to create a new workspace
  - In the new workspace `jj edit aa`
  - Manually copy changes from new workspace to `ba`
- `jj squash` to abandon `ba` by moving resolution into `b`

⚫ Revision (`d`): `jj new c`

- Verify unit tests now pass
- `jj describe -m "Write functional tests"`
- Write the functional tests

# todo: talk about how to push all that.
