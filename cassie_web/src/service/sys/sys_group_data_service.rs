







pub struct SysGroupDataService {}
impl Default for SysGroupDataService {
    fn default() -> Self {
        SysGroupDataService {}
    }
}
// impl CrudService<SysGroupData, SysGroupDataDTO, SysGroupDataQuery> for SysGroupDataService {
//     fn get_wrapper(arg: &SysGroupDataQuery) -> rbatis::wrapper::Wrapper {
//         let rb = APPLICATION_CONTEXT.get::<Rbatis>();
//         rb.new_wrapper()
//     }
//     fn set_save_common_fields(&self, common: CommonField, data: &mut SysGroupData) {}
// }
