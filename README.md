# autod

- [x] Create oneshot service with `autod <command>`
- [x] Create service to run on SystemdTarget `autod <command> on <SystemdTarget>`
- [x] Create service to run on SystemdTimer `autod <command> when <TimePattern>`
- [ ]  `run` and `enable`
- [ ] Description
- [ ] User/System
- [ ] other fields

## Open Questions

- [ ] Should OneShot be the default?


## Fields to Consider

### Type

- [ ] simple
- [ ] oneshot
(- [ ] forking)

### Target
- [ ] default.target
- [ ] timers.target
