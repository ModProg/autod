# autod

- [x] Create oneshot service with `autod <command>`
- [x] Create service to run on SystemdTarget `autod <command> on <SystemdTarget>`
- [x] Create service to run on SystemdTimer `autod <command> when <TimePattern>`
- [ ] `run` and `enable`
- [ ] Description
- [ ] User/System
- [ ] other fields
- [ ] Create Timer for existing Service
- [ ] Add Timer entry to existing Timer
- [ ] Add "ON" event to existing Service

## Open Questions

- [x] Should OneShot be the default? no, I think.
- [ ] Do timers need, `[Install]\n WantedBy=timerts.target`?

## Fields to Consider

### Type

- [ ] simple
- [ ] oneshot

(- [ ] forking)

### Target

- [ ] default.target
- [ ] timers.target

## Timer

Multiple entries, separated by `,`

### Timespan

#### Units

Only English, test with `systemd-analize timespan`, enforce no unit = seconds

- usec, us, µs
- msec, ms
- seconds, second, sec, s
- minutes, minute, min, m
- hours, hour, hr, h
- days, day, d
- weeks, week, w
- months, month, M (defined as 30.44 days)
- years, year, y (defined as 365.25 days)

#### Fields

| Systemd           | Autod           | Description                     |
| ----------------- | --------------- | ------------------------------- |
| OnActiveSec       | SinceTimer, ST  | the timer last ran              |
| OnBootSec         | SinceBoot, SB   | boot                            |
| OnStartupSec      | SinceLogin, SL  | Service manager started (login) |
| OnUnitActiveSec   | SinceSrvAct, SA | Service was last started        |
| OnUnitInactiveSec | SinceSrvEnd, SE | Service last stopped            |
| "OnCalendar"      | IN              | Run Service once                |
| "OnCalendar"      | InAt, IA        | Run Service in smth at smth     |
| "OnCalendar"      | Every, EV       | Run the service every smth      |
| "OnCalendar"      | EveryAt, EA     | Run every smth at smth          |

##### InAt

- `In 5d At 4am` => in 5 days at 4am
- `InAt 5y 05-01 00:04` => in 5 years at 05-01 00:04
- `I 2w At Mon` => in 2 weeks at Monday (prop midnight)
- `I 3M A *:*:2` => in 3 months every minute at 2 seconds (but for how long?)

The same for EveryAt

**TODO** Implement some form of duration

#### Shorthands for EV

| Systemd  | Autod |
| -------- | ----- |
| minutely | mly   |
| hourly   | hly   |
| daily    | dly   |
| monthly  | Mly   |
| weekly   | wly   |
| yearly   | yly   |

### Calendar

OnCalendar => C

- Some form of universal from to ``
- From A to B Every C mins

## Refactoring

- Remove a lot of `Option<>` and replace with the `0` variant (is
  already the behavior)
