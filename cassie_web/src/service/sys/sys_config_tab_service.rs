







pub struct SysConfigTabService {}
impl Default for SysConfigTabService {
    fn default() -> Self {
        SysConfigTabService {}
    }
}
// impl CrudService<SysConfigTab, SysConfigTabDTO, SysConfigTabQuery> for SysConfigTabService {
//     fn get_wrapper(arg: &SysConfigTabQuery) -> rbatis::wrapper::Wrapper {
//         let rb = APPLICATION_CONTEXT.get::<Rbatis>();
//         rb.new_wrapper()
//     }
//     fn set_save_common_fields(&self, common: CommonField, data: &mut SysConfigTab) {}
// }
