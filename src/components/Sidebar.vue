<template>
  <el-menu
      default-active="1"
      class="el-menu-vertical"
      :collapse="isCollapse"
      background-color="#545c64"
      text-color="#fff"
      active-text-color="#ffd04b"
  >
    <el-menu-item index="0" @click="$emit('toggle-collapse')">
      <el-icon>
        <FontAwesomeIcon :icon="isCollapse ? 'expand' : 'fas fa-database'" />
      </el-icon>
      <span>{{ isCollapse ? '展开' : 'ESManager' }}</span>
    </el-menu-item>

    <el-tree
        :data="treeData"
        :props="defaultProps"
        @node-click="handleNodeClick"
        node-key="id"
        default-expand-all
        :expand-on-click-node="false"
    >
      <template #default="{ node, data }">
        <span class="custom-tree-node">
          <FontAwesomeIcon v-if="data.icon" :icon="['fas', data.icon]" class="node-icon" />
          <span>{{ node.label }}</span>
        </span>
      </template>
    </el-tree>
  </el-menu>
</template>

<script>
import { Fold, Expand } from '@element-plus/icons-vue'
import {invoke} from "@tauri-apps/api/core";

// 图标映射 - 根据节点类型返回对应的图标名称
const iconMap = {
  connection: 'server',
  indexParent: 'database',
  index: 'folder',
  field: 'file-alt'
}

async function getConnectionConfig(){
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  return await invoke("get_all_connections", {  });
}

async function getAllIndexs(connKey){
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  return await invoke("get_all_indexs", {connKey: connKey});
}

// 获取索引下面所有的字段
async function getColumns(connKey,indexName){
  return await invoke("get_columns_by_index",{connKey: connKey,indexName:indexName})
}

async function getDocument(connKey,indexName){
  return await invoke("get_all_docment",{connKey: connKey,indexName:indexName})
}

async function getDocumentByCondition(connKey,indexName,condition){
  return await invoke("query_document_by_query_param",{connKey: connKey,indexName:indexName,condition:condition})
}

export default {
  components: { Fold, Expand },
  created(){
    let promise = getConnectionConfig();
    promise.then(resp => {
      let rst = JSON.parse(resp);
      let da = rst.data;
      console.log(JSON.stringify(da))

      for (let i = 0;i < da.length;i++){
        let childNodes = [{
              id: 11, 
              label: '索引',
              icon: iconMap.index,
              parentNodeLabel:da[i].connect_name,
              isIndexParent:true,
              children:[
                {id:111,label:'index1',icon:iconMap.field}
              ] 
        }]
        let node = {
          id:i+1,
          label:da[i].connect_name,
          icon:iconMap.connection,
          children:childNodes
        };
        this.treeData.push(node);
      }
    })
  },
  props: {
    isCollapse: Boolean
  },
  data() {
    return {
      treeData: [
      ],
      defaultProps: {
        children: 'children',
        label: 'label'
      },
      queryParam:{
        "query":{
          "match_all":{}
        },
        "from":0,
        "size":10
      }
    }
  },
  methods: {
    handleNodeClick(data) {
      // this.$emit('node-click', data)
      console.log("click data:"+JSON.stringify(data))
      if(data?.isIndexParent != undefined && data.isIndexParent){
        this.openIndex(data)
      }

      if(data?.isIndexNode != undefined && data.isIndexNode){
        this.loadTableData(data).then(tableData => {
          console.log("组装好的数据:", tableData);
          this.$emit('node-click', tableData)
        })
      }
    },
    async loadTableData(data){
      console.log("前端转换的json：",JSON.stringify(this.queryParam));
      const [columnsResp, documentsResp] = await Promise.all([
          getColumns(data.connectName, data.label),
          getDocumentByCondition(data.connectName, data.label,JSON.stringify(this.queryParam))
        ]);

        const parseResponse = (resp) => {
          const result = JSON.parse(resp);
          if (result.code === "500") {
            throw new Error(result.message);
          }
          return result.data;
        };
        const columnsData = parseResponse(columnsResp);
        console.log("输出字段的相关数据，查看有哪些类型：",columnsData);
        const documentsData = parseResponse(documentsResp);

        // 3. 转换数据结构
        const tableData = {
          columns: columnsData.map(ele => ({
            prop: ele.column_name,
            label: ele.column_name,
            column_type:ele.column_type
          })),
          data: documentsData.documents.map(ele => ele._source),
          total:documentsData.total,
          indexName:data.label,
          connectName:data.connectName
        };
        
        return tableData;
    },
    openIndex(orignData){
      console.log("获取所有的索引:"+JSON.stringify(orignData.parentNodeLabel))
      let promise =  getAllIndexs(orignData.parentNodeLabel);
        promise.then(resp =>{
          let rst = JSON.parse(resp);
          if(rst.code != '200'){
            this.$message.error('连接失败，原因：'+rst.message);
            return;
          }
          let children = [];
          for(const ele of rst.data){
            let node = {
              id:ele.uuid,
              label:ele.index,
              connectName:orignData.parentNodeLabel,
              type:'product-list',
              isIndexNode:true,
              icon:iconMap.field
            }
            children.push(node);
          }
          orignData.children = children;
        })
      
    }
  }
}
</script>

<style>
.el-menu-vertical {
  height: 100%;
  border-right: none;
  background: linear-gradient(180deg, #2c3e50, #1a2530) !important;
  transition: all 0.3s ease;
}
.el-menu-vertical:not(.el-menu--collapse) {
  width: 100%;
}

.el-menu-item {
  height: 56px !important;
  display: flex !important;
  align-items: center;
  font-size: 15px;
  transition: all 0.2s ease !important;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05) !important;
}

.el-menu-item:hover {
  background-color: rgba(52, 152, 219, 0.2) !important;
}

.el-menu-item i {
  margin-right: 12px;
  font-size: 18px;
}

.el-tree {
      background-color: transparent !important;
      color: #e0f7fa;
      margin-top: 10px;
      padding: 0 10px;
    }
    
    .el-tree-node {
      margin: 8px 0;
    }
    
    .el-tree-node__content {
      height: 40px !important;
      border-radius: 6px;
      transition: all 0.2s ease;
      padding-left: 16px !important;
    }
    
    .el-tree-node__content:hover {
      background-color: rgba(41, 128, 185, 0.25) !important;
      transform: translateX(3px);
    }
    
    .el-tree-node.is-current > .el-tree-node__content {
      background-color: rgba(46, 204, 113, 0.15) !important;
      color: #64ffda;
      font-weight: 500;
      border-left: 3px solid #64ffda;
    }
    
    .custom-tree-node {
      flex: 1;
      display: flex;
      align-items: center;
      font-size: 14px;
      padding: 0 8px;
    }
    
    .node-icon {
      margin-right: 10px;
      font-size: 16px;
      width: 20px;
      text-align: center;
    }
    
    .connection-icon {
      color: #3498db;
    }
    
    .indexes-icon {
      color: #9b59b6;
    }
    
    .index-icon {
      color: #2ecc71;
    }
    
    .collapse-toggle {
      background: rgba(44, 62, 80, 0.9) !important;
      border-bottom: 1px solid rgba(255, 255, 255, 0.1) !important;
    }
    
    .sidebar-title {
      padding: 20px 20px 10px;
      display: flex;
      align-items: center;
      color: #64ffda;
      font-size: 18px;
      font-weight: 600;
    }
    
    .sidebar-title i {
      margin-right: 10px;
      font-size: 20px;
    }
    
    .status-bar {
      padding: 15px 20px;
      background: rgba(0, 0, 0, 0.2);
      margin-top: auto;
      display: flex;
      align-items: center;
      font-size: 13px;
      color: #81d4fa;
      border-top: 1px solid rgba(255, 255, 255, 0.05);
    }
    
    .status-indicator {
      width: 10px;
      height: 10px;
      border-radius: 50%;
      background: #2ecc71;
      margin-right: 10px;
      box-shadow: 0 0 8px #2ecc71;
    }
    
    .logo {
      display: flex;
      align-items: center;
      padding: 20px;
      border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }
    
    .logo-icon {
      font-size: 28px;
      color: #64ffda;
      margin-right: 12px;
    }
    
    .logo-text {
      font-size: 22px;
      font-weight: 700;
      background: linear-gradient(90deg, #64ffda, #4fc3f7);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    
    @media (max-width: 900px) {
      .container {
        flex-direction: column;
      }
      
      .sidebar-container {
        width: 100%;
        max-height: 500px;
      }
    }
</style>