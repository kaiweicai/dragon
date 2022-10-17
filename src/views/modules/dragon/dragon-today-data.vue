<template>
  <el-dialog :visible.sync="visible" :title="!dataForm.id ? $t('add') : $t('update')" :close-on-click-modal="false"
    :close-on-press-escape="false" width="80%">

      <!-- <div v-for="(item, idx) in dataForm.data" header-align="center" align="center">
          {{idx}}:
          <ul >
            <ui v-for="(user) in item">
              {{ user}}
            </ui>
          </ul>
        </div> -->

      <el-table :data="dataForm.data" border style="width: 100%;">
        <el-table-column prop="amount" label="下单量" header-align="center" align="center" width="100">
        </el-table-column>
        <el-table-column prop="dragonDataVec" label="用户列表" header-align="center" align="center">
          <template slot-scope="scope">
            <el-table :data="scope.row.dragonDataVec" stripe>
              <el-table-column prop="no" label="编号"></el-table-column>
              <el-table-column prop="name" label="姓名"></el-table-column>
              <el-table-column prop="amount" label="下单量"></el-table-column>
              <el-table-column prop="disable" label="失效" :formatter="formatBoolean" :show-overflow-tooltip="true">
              </el-table-column>
              <el-table-column label="操作" fixed="right" header-align="center" align="center" width="150">
                <template slot-scope="scope">
                  <!-- <el-button   type="text" size="small" @click="addOrUpdateHandle(scope.row.id)">{{ $t('update') }}</el-button> -->
                  <el-button type="text" size="small" @click="disable(scope.$index,scope.row)">{{ $t('delete') }}
                  </el-button>
                  <el-button type="text" size="small" @click="deleteHandle(scope.row.no)">{{ $t('delete') }}</el-button>
                </template>
              </el-table-column>
            </el-table>
          </template>
        </el-table-column>
      </el-table>
    <template slot="footer">
      <el-button @click="visible = false">{{ $t('cancel') }}</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle()">{{ $t('confirm') }}</el-button>
    </template>
  </el-dialog>
</template>

<script>
import debounce from 'lodash/debounce'
const Base64 = require('js-base64').Base64;
// import { isEmail, isMobile } from '@/utils/validate'
export default {
  data() {
    return {
      visible: false,
      dataForm: {
        id: '',
        content: '',
        data: {},
      }
    }
  },
  methods: {
    init(id) {
      this.visible = true
      this.$nextTick(() => {
        // this.$refs['dataForm'].resetFields()
        Promise.all([
          this.loadTodayData(id)
        ]).then(() => {
          console.log("初始化成功!");
        })
      })
    },
    // 获取信息
    loadTodayData(id) {
      this.$http.get(`/dragon/todaydata/${id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm.data = res.data;
      }).catch((e) => {
        console.log('发生错误' + e);
      })
    },
    formatBoolean: function (row, index) {
      var ret = ''
      if (row.is_admin == true) {
        ret = "yes" //根据自己的需求设定
      } else {
        ret = "no"
      }
      return ret;
    },
    //设置当前的接龙为无效
    disable(index,row) {
      alert(JSON.stringify(row));
      row.disable = true;
      alert(this.dataForm.data[0].dragonDataVec[0].disable);
      this.$set(this.dataForm.data.dragonDataVec,index,row);
    },

    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http[!this.dataForm.id ? 'post' : 'put']('/dragon', {
          content: Base64.encode(this.dataForm.content),
          // createDate:this.dataForm.createDate,
        }).then(({ data: res }) => {
          if (res.code !== 0) {
            return this.$message.error(res.msg)
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.visible = false
              this.$emit('refreshDataList')
            }
          })
        }).catch(() => { })
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>

<style lang="scss">
.mod-sys__user {
  .dept-list {

    .el-input__inner,
    .el-input__suffix {
      cursor: pointer;
    }
  }

  .role-list {
    .el-select {
      width: 100%;
    }
  }
}
</style>
