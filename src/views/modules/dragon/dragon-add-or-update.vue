<template>
  <el-dialog :visible.sync="visible" :title="!dataForm.id ? $t('add') : $t('update')" :close-on-click-modal="false" :close-on-press-escape="false">
    <el-form :model="dataForm"  ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()" label-width="120px">
      <el-form-item  prop="content" :label="$t('dragon.content')">
        <el-input type="textarea" v-model="dataForm.content" :placeholder="$t('dragon.content')"></el-input>
      </el-form-item>
      <el-form-item prop="createDate" :label="$t('dragon.createDate')" :class="{ 'is-required': !dataForm.id }">
          <el-date-picker
            v-model="dataForm.createDate"
            align="right"
            type="date"
            :placeholder="$t('dragon.createDate')"
            >
          </el-date-picker>
        </el-form-item>
    </el-form>
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
  data () {
    return {
      visible: false,
      roleList: [],
      roleIdListDefault: [],
      postList: [],
      dataForm: {
        id: '',
        content: '',
        createDate: '',
      }
    }
  },
  // computed: {
  //   dataRule () {
  //     var validatePassword = (rule, value, callback) => {
  //       if (!this.dataForm.id && !/\S/.test(value)) {
  //         return callback(new Error(this.$t('validate.required')))
  //       }
  //       callback()
  //     }
  //     var validateEmail = (rule, value, callback) => {
  //       if (!isEmail(value)) {
  //         return callback(new Error(this.$t('validate.format', { 'attr': this.$t('user.email') })))
  //       }
  //       callback()
  //     }
  //     var validateMobile = (rule, value, callback) => {
  //       if (!isMobile(value)) {
  //         return callback(new Error(this.$t('validate.format', { 'attr': this.$t('user.mobile') })))
  //       }
  //       callback()
  //     }
  //     return {
  //       username: [
  //         { required: true, message: this.$t('validate.required'), trigger: 'blur' }
  //       ],
  //       password: [
  //         { validator: validatePassword, trigger: 'blur' }
  //       ],
  //       real_name: [
  //         { required: true, message: this.$t('validate.required'), trigger: 'blur' }
  //       ],
  //       email: [
  //         { required: true, message: this.$t('validate.required'), trigger: 'blur' },
  //         { validator: validateEmail, trigger: 'blur' }
  //       ],
  //       mobile: [
  //         { required: true, message: this.$t('validate.required'), trigger: 'blur' },
  //         { validator: validateMobile, trigger: 'blur' }
  //       ]
  //     }
  //   }
  // },
  methods: {
    init () {
      this.visible = true
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        // Promise.all([
        //   this.getRoleList()
        // ]).then(() => {
        //   if (this.dataForm.id) {
        //     this.getInfo()
        //   }
        // })
      })
    },
    // 获取角色列表
    getRoleList () {
      return this.$http.get('/role/list').then(({ data: res }) => {
        if (res.code != 0) {
          return this.$message.error(res.msg)
        }
        this.roleList = res.data
      }).catch(() => {})
    },
    // 获取信息
    getInfo () {
      this.$http.get(`/user/${this.dataForm.id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data,
          role_id: 0
        }
        // 角色配置, 区分是否为默认角色
        this.roleIdListDefault.push(res.data.role_id)
      }).catch(() => {})
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http[!this.dataForm.id ? 'post' : 'put']('/dragon', {
          content:Base64.encode(this.dataForm.content),
          createDate:this.dataForm.createDate,
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
        }).catch(() => {})
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
