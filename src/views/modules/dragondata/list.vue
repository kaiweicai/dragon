<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-sys__user">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-date-picker
            v-model="dataForm.createDate"
            align="right"
            type="date"
            placeholder="选择日期"
            >
          </el-date-picker>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button  type="primary" @click="addOrUpdateHandle()">{{ $t('add') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button  type="danger" @click="deleteHandle()">{{ $t('deleteBatch') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button  type="info" @click="exportHandle()">{{ $t('export') }}</el-button>
        </el-form-item>
      </el-form>
      <el-table
        v-loading="dataListLoading"
        :data="dataList"
        border
        @selection-change="dataListSelectionChangeHandle"
        @sort-change="dataListSortChangeHandle"
        style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="no" label="序号" sortable="custom" header-align="center" align="center"></el-table-column>
        <el-table-column prop="name" label="姓名" sortable="custom" header-align="center" align="center"></el-table-column>
        <el-table-column prop="amount" label="下单量" sortable="custom" header-align="center" align="center"></el-table-column>
        <el-table-column prop="prior" label="优先级" :formatter="formatBoolean" sortable="custom" header-align="center" align="center"></el-table-column>
        <el-table-column prop="disable" label="取消" :formatter="formatBoolean" sortable="custom"  header-align="center" align="center"></el-table-column>
        <el-table-column prop="createDate" label="创建日期"  header-align="center" align="center"></el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <!-- <el-button   type="text" size="small" @click="addOrUpdateHandle(scope.row.id)">{{ $t('update') }}</el-button> -->
            <el-button type="text" size="small" @click="gentodaydata(scope.row.no)">失效/有效</el-button>
            <el-button type="text" size="small" @click="deleteHandle(scope.row.id)">{{ $t('delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
      <el-pagination
        :current-page="page"
        :page-sizes="[10, 20, 50, 100]"
        :page-size="limit"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="pageSizeChangeHandle"
        @current-change="pageCurrentChangeHandle">
      </el-pagination>
      <!-- 弹窗, 新增 / 修改 -->
      <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getDataList"></add-or-update>
      <TodayData v-if="todayDataVisible" ref="todayData" ></TodayData>
    </div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
// import AddOrUpdate from './dragon-add-or-update';
// import TodayData from './dragon-today-data';
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/dragondata/list',
        getDataListIsPage: false,
        deleteURL: '/dragondata',
        deleteIsBatch: false,
        exportURL: '/user/export'
      },
      dataForm: {
        content: '',
        createDate: '',
      },
      todayDataVisible: false,
    }
  },
  components: {
    // AddOrUpdate,
    // TodayData,
  },
  created () {
  },
  methods: {
    formatBoolean: function (row, index) {
      var ret = ''
      if (row.is_admin == true) {
        ret = "yes" //根据自己的需求设定
      } else {
        ret = "no"
      }
      return ret;
    },
    gentodaydata(no){
      alert(no);
      this.todayDataVisible = true;
      this.$nextTick(() => {
        // this.$refs.data.dataForm.smsCode = row.smsCode
        // this.$refs.todayData.init(no);
      })
    }
  }
}
</script>
