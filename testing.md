
# Testing

## Task: Add a config field that repeats certain events on an interval

I like this example because its a really fertile example for testing, particularly integration testing.

Unit:

- Configuration: messaging around invalid config
- Timings of events, injection of config to trigger events

TBD:

- Api to dispatch events

## Config injection: adding configurable toggles to your application that would not be appropriate to build into the UI.

- Run integration tests with my cli where the interval is less than 1m.
- Integration test when there is a high startup cost that is unnecessary for a specific integration test, switch that high-cost feature off.
- Stub out some separate service and return some known data
  - Suppose each event should be sent to some grpc application and it determines if the event should be published to a wider audience
  - We may want to stub that code to just verify the wrapper around the grpc service gets called, we assume the grpc generated service works fine.
  - Perhaps a service which labels transactions as sus, if sus then dont dispatch the event
- Configuration: instead of saving a file and waiting for the program to reload, allow injection thru variations
- Licensing: Manipulate expiration time to verify messaging
  - Are you valid?
  - Are you in a warning period
  - Are you in a grace period
  - Are you expired

## State sync: You have a multitude of disparate places which hold state, need to setup state st. its all in sync

- User specific state
  - User state stored durably
  - User state 1-1 corr with which topic of events they see
  - Test consists of setting up user in specific state, test behaves as expected. Measurements?

## Failure scenarios

- Calls to external services return error
- Fallback experiences?
- Syscall failures?
- Any function that may error, specifically component sync areas: see channel send and receive.
