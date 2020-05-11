# 🎮 Vigem bindings for Rust! 🎮

### It's WIP

### Warning
I cant link this lib (**need help**) so you have to move [dll for your  arch]("https://github.com/DuckerMan/vigem/blob/master/dlls/") to your `.exe` file catalog

Other problem, that bindgen can generate right binding for `EVT_VIGEM_X360_NOTIFICATION` and `EVT_VIGEM_DS4_NOTIFICATION` so I also need help

### How to use?
Check examples directory

# Functions:

- [x] `vigem_alloc`
- [x] `vigem_free`
- [x] `vigem_connect`
- [x] `vigem_disconnect`
- [x] `vigem_target_x360_alloc`
- [x] `vigem_target_ds4_alloc`
- [x] `vigem_target_free`
- [x] `vigem_target_add`
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
- [x] `vigem_target_x360_update`
- [ ] `vigem_register_notification` - In progress
- [ ] `vigem_target_add_async` - I dont know how to realize it (need help)