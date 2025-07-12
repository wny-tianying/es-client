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
        <component :is="isCollapse ? 'Expand' : 'Fold'" />
      </el-icon>
      <span>{{ isCollapse ? '展开' : '收起' }}</span>
    </el-menu-item>

    <el-tree
        :data="treeData"
        :props="defaultProps"
        @node-click="handleNodeClick"
        node-key="id"
        default-expand-all
        :expand-on-click-node="false"
    />
  </el-menu>
</template>

<script>
import { Fold, Expand } from '@element-plus/icons-vue'
import {invoke} from "@tauri-apps/api/core";

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
        this.treeData[i].id = i+1;
        this.treeData[i].label = da[i].connect_name;
        let childNodes = [{
              id: 11, 
              label: '索引',
              parentNodeLabel:da[i].connect_name,
              isIndexParent:true,
              children:[
                {id:111,label:'index1'}
              ] 
        }]
        this.treeData[i].children = childNodes;
      }
    })
  },
  props: {
    isCollapse: Boolean
  },
  data() {
    return {
      treeData: [
        {
          id: 1,
          label: '销售数据',
          children: [
            { 
              id: 11, 
              label: '索引',
              parentNodeLabel:"default",
              isIndexParent:true,
              children:[
                {id:111,label:'index1'}
              ] 
            },
            { 
              id: 12, 
              label: '季度销售', 
              type: 'sales-quarter' 
            },
            { 
              id: 13, 
              label: '年度销售', 
              type: 'sales-year' 
            }
          ]
        },
        {
          id: 2,
          label: '产品管理',
          children: [
            { id: 21, label: '产品列表', type: 'product-list' },
            { id: 22, label: '库存管理', type: 'product-stock' }
          ]
        },
        {
          id: 3,
          label: '客户管理',
          type: 'customer'
        }
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
.el-menu-vertical:not(.el-menu--collapse) {
  width: 250px;
  min-height: 400px;
}

.el-tree {
  background-color: transparent;
  color: white;
  margin-top: 10px;
}

.el-tree-node__content:hover {
  background-color: rgba(255, 255, 255, 0.1) !important;
}
</style>