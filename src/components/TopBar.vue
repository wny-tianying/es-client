<template>
  <el-menu
      mode="horizontal"
      background-color="#409EFF"
      text-color="#fff"
      active-text-color="#ffd04b"
      @select="handleSelect"
  >
    <el-sub-menu index="file">
      <template #title>文件</template>
      <el-menu-item index="file-new" @click="createNewConnection()">新建</el-menu-item>
      <el-menu-item index="file-open">打开</el-menu-item>
      <el-menu-item index="file-save">保存</el-menu-item>
    </el-sub-menu>

    <el-sub-menu index="edit">
      <template #title>编辑</template>
      <el-menu-item index="edit-undo">撤销</el-menu-item>
      <el-menu-item index="edit-redo">重做</el-menu-item>
    </el-sub-menu>

    <el-sub-menu index="view">
      <template #title>视图</template>
      <el-menu-item index="view-refresh">刷新</el-menu-item>
      <el-menu-item index="view-fullscreen">全屏</el-menu-item>
    </el-sub-menu>

    <el-sub-menu index="setting">
      <template #title>设置</template>
      <el-menu-item index="setting-preference">首选项</el-menu-item>
      <el-menu-item index="setting-theme">主题</el-menu-item>
    </el-sub-menu>

    <el-menu-item index="help">帮助</el-menu-item>
  </el-menu>

  <el-dialog v-model="showCreateConnectionFlag" title="Shipping address" width="500">
    <el-form :model="form">
      <el-form-item label="连接名称" :label-width="formLabelWidth">
        <el-input v-model="form.connect_name" autocomplete="off" />
      </el-form-item>
      <el-form-item label="ip地址" :label-width="formLabelWidth">
        <el-input v-model="form.ip" autocomplete="off" />
      </el-form-item>
      <el-form-item label="端口" :label-width="formLabelWidth">
        <el-input v-model="form.port" autocomplete="off" />
      </el-form-item>
      <el-form-item label="协议" :label-width="formLabelWidth">
        <el-select v-model="form.protocol" placeholder="Please select a zone">
          <el-option label="http" value="http" />
          <el-option label="https" value="https" />
        </el-select>
      </el-form-item>
      <el-form-item label="用户名" :label-width="formLabelWidth">
        <el-input v-model="form.username" autocomplete="off" />
      </el-form-item>
      <el-form-item label="密码" :label-width="formLabelWidth">
        <el-input v-model="form.password" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="testConnect()">test</el-button>
        <el-button @click="showCreateConnectionFlag = false">Cancel</el-button>
        <el-button type="primary" @click="createConnection()">
          Confirm
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script>

import {invoke} from "@tauri-apps/api/core";

async function testConnect(form){
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  return await invoke("test_connect_config", { data: JSON.stringify(form) });
}

async function createConnection(form){
  return await invoke("create_new_connection",{data: JSON.stringify(form)});
}
export default {
  data(){
    return{
      showCreateConnectionFlag:false,
      form:{
        ip:"192.168.1.124",
        port:"9200",
        protocol:"http",
        username:"elastic",
        password:"123456",
        connect_name:"first",
      }
    }

  },
  methods: {
    handleSelect(key) {
      this.$emit('menu-click', key);
      console.log("select button:{}",key);
    },
    createNewConnection(){
      this.showCreateConnectionFlag = true;
    },
    testConnect(){
      let promiseObj =testConnect(this.form);

      promiseObj.then(resp=>{
        console.log("response:{}",resp);
        let rst = JSON.parse(resp);
        if (rst.code === "200"){
          this.$message({
            message: '连接成功',
            type: 'success'
          });
        }else {
          this.$message.error('连接失败，原因：'+rst.message);
        }
      })
    },
    createConnection(){


      let promise = createConnection(this.form);
      promise.then(result=>{
        console.log("后端的响应结果："+result);
        let rst = JSON.parse(resp);
        if (rst.code === "200"){
          this.$message({
            message: '连接成功',
            type: 'success'
          });
        }else {
          this.$message.error('连接失败，原因：'+rst.message);
        }
      })

    }
  }
}
</script>

<style>
.el-menu--horizontal {
  border-bottom: none;
}
</style>