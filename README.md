# 🎮 ViGEm bindings for Rust! 🎮
[![](http://meritbadge.herokuapp.com/vigem)](https://crates.io/crates/vigem)
[![documentation (docs.rs)](https://docs.rs/vigem/badge.svg)](https://docs.rs/vigem)

### What is it?

>[ViGEm](https://github.com/ViGEm/ViGEmBus) is Virtual Gamepad Emulation Framework.

These bindings are based on [ViGEm client](https://github.com/ViGEm/ViGEmClient)
### How to use?
Check [examples](https://github.com/DuckerMan/vigem/tree/master/examples) or [read the docs](https://docs.rs/vigem)


![Cool Gamepad Image](https://forums.vigem.org/assets/uploads/files/1557594978383-6c4d9c51-5644-40ec-bd42-a54f022729db-image.png)


# Functions:

- [x] `vigem_alloc`
- [x] `vigem_free`
- [x] `vigem_connect`
- [x] `vigem_disconnect`
- [x] `vigem_target_x360_alloc`
- [x] `vigem_target_ds4_alloc`
- [x] `vigem_target_free`
- [x] `vigem_target_add`
- [x] `vigem_target_add_async` - Need help to add async/await support
- [x] `vigem_target_remove`
- [x] `vigem_target_get_index`
- [x] `vigem_target_get_type`
- [x] `vigem_target_is_attached`
- [x] `vigem_target_x360_unregister_notification`
- [x] `vigem_target_ds4_unregister_notification`
- [x] `vigem_target_set_vid`
- [x] `vigem_target_set_pid`
- [x] `vigem_target_get_vid`
- [x] `vigem_target_get_pid`
- [x] `vigem_target_x360_get_user_index`
- [x] `vigem_register_notification`
- [x] `vigem_target_update`
- [x] `vigem_target_send_report`

# Plans:
- [x] [Documentation](https://docs.rs/vigem)
- [x] [Split to `-sys` ](https://github.com/DuckerMan/vigem/tree/master/vigem-sys)
- [x] Add `tests`
- [] Figure out why `userdata` can be another type and value
- [ ] Auto bindings generation
- [ ] **Finish `Life is Strange 2` game**