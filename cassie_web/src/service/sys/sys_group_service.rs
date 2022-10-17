







pub struct SysGroupService {}
impl Default for SysGroupService {
    fn default() -> Self {
        SysGroupService {}
    }
}
// impl CrudService<SysGroup, SysGroupDTO, SysGroupQuery> for SysGroupService {
//     fn get_wrapper(arg: &SysGroupQuery) -> rbatis::wrapper::Wrapper {
//         let rb = APPLICATION_CONTEXT.get::<Rbatis>();
//         rb.new_wrapper()
//     }
//     fn set_save_common_fields(&self, common: CommonField, data: &mut SysGroup) {}
// }
