<template>
  <div class="data-table-container">
    <el-card class="table-card">
      <template #header>
        <div class="card-header">
          <span>{{ tableTitle }}</span>
          <el-button type="primary" size="small" @click="handleExport">导出数据</el-button>
        </div>
      </template>

      <div class="table-wrapper">
        <el-table
            :data="tableData"
            border
            style="width: 100%"
            height="250"
        >
          <el-table-column
              v-for="column in columns"
              :key="column.prop"
              :prop="column.prop"
              :label="column.label"
              :width="column.width"
          />
        </el-table>
      </div>

      <div class="pagination-wrapper">
        <el-pagination
            :page-size="pageSize"
            :page-sizes="pageSizes"
            :total="total"
            :current-page="currentPage"
            layout="sizes,prev, pager, next"
            @size-change="handleSizeChange"
            @current-change="handlePageChange"
        />
      </div>
    </el-card>
  </div>
</template>

<script>
import {invoke} from "@tauri-apps/api/core";


async function queryDocument(connKey,indexName,condition){
  return await invoke("query_docment",{connKey: connKey,indexName:indexName,condition})
}

async function getDocumentByCondition(connKey,indexName,condition){
  return await invoke("query_document_by_query_param",{connKey: connKey,indexName:indexName,condition:condition})
}


export default {
  props: {
    currentNode: Object,
    filters: Array
  },
  data() {
    return {
      indexName:"",
      connectName:"",
      currentPage: 1,
      tableData: [],
      pageSize: 10,
      pageSizes: [5,10, 20, 50, 100], // 可选的每页大小
      tableHeight: '100%',
      tableConfig: {},
      defaultConfig: {
        title: '请选择数据类别',
        columns: [
          { prop: 'tip', label: '提示' }
        ],
        data: [
          { tip: '请从左侧树形菜单中选择要查看的数据类别' }
        ]
      }
    }
  },
  computed: {
    tableTitle() {
      return this.defaultConfig.title;
    },
    columns() {
      if (!this.currentNode) return this.defaultConfig.columns
      this.tableData = this.currentNode?.data || [];
      this.indexName = this.currentNode.indexName;
      this.connectName = this.currentNode.connectName;
      return this.currentNode.columns;
    },
    allData() {
      console.log("表格数据刷新")
      if(!this.currentNode){
        this.tableData = this.currentNode?.data || [];
      }
      return this.tableData;
    },
    filteredData() {
      console.log("filteredData刷新")
      if (!this.filters || this.filters.length === 0) {
        return this.allData.slice((this.currentPage - 1) * this.pageSize, this.currentPage * this.pageSize)
      }
    },
    total() {
      if (this.currentNode) {
        return this.currentNode.total;
      }

      if (!this.filters || this.filters.length === 0) {
        return this.allData.length
      }
    }
  },
  watch: {
    filters: {
      deep: true,
      immediate: true,
      handler() {
        // 处理过滤逻辑并发送请求
        console.log("filters发生了变化？")
        if(!this.filters || this.filters.length === 0){
          return;
        }
        this.handleFilterChange();
      }
    }
  },
  methods: {
    handlePageChange(page) {
      this.currentPage = page
    },
    handleFilterChange() {
      // 构建查询参数
      let body = {
        query:{
          bool:{
            should:[]
          }
        }
      };
      let singleGroup = {bool:{must:[]}};
      this.filters.forEach((ele,index)=>{
        if(ele.connector == 'and'){
          singleGroup.bool.must.push(this.handleCondition(ele));
        }else{
          body.query.bool.should.push(singleGroup);
          singleGroup = {bool:{must:[]}};
          singleGroup.bool.must.push(this.handleCondition(ele));
        }
      })
      body.query.bool.should.push(singleGroup);
      let queryEsParam = {
        query:body.query,
        from:0,
        size:this.pageSize
      };
      
      this.sendReqAndHandleResp(queryEsParam, this.pageSize);
    },
    // 处理每一个元素应该组成es的script语句
    handleCondition(condition) {
      switch (condition.operator) {
        case '=': {
          return {
            term: {
              [condition.field]: condition.value
            }
          };
        }
        case '>': {
          return {
            range: {
              [condition.field]: {
                gt: condition.value  // 分号改为逗号
              }
            }
          };
        }
        case '<': {
          return {
            range: {
              [condition.field]:{
                lt: condition.value  // 分号改为逗号
              }
            }
          };
        }
        case 'like': {
          return {
            match: {
              [condition.field]: {
                query: condition.value,
                fuzziness: "AUTO"
              }
            }
          };
        }
        case '!=':{
          return {
            bool:{
              must_not:{
                term:{
                  [condition.field]:condition.value
                }
              }
            }
          };
        }
        case 'not null':{
          return {bool: {
              must: [
                { exists: { field: condition.field} }  // 确保字段存在
              ],
              must_not: [
              { term: { [condition.field]: "" } }         // 排除空字符串
            ]
          }
        };}
        default:
          return null;
      }
    },
    handleSizeChange(pageSize) {
      // 组装请求参数
      let queryEsParam = {
        query:{
          match_all:{}
        },
        from:0,
        size:pageSize
      }
      this.sendReqAndHandleResp(queryEsParam,pageSize); 
    },
    sendReqAndHandleResp(queryEsParam,pageSize){
      let rest = getDocumentByCondition(this.connectName,this.indexName,JSON.stringify(queryEsParam));
      rest.then(resp=>{
        let rst = JSON.parse(resp);
        if(rst.code != "200"){
          this.$message.error('查询失败：'+rst.message);
          return;
        }
        console.log("分页查询的数据：",rst.data);
        
        this.tableData = rst.data.documents.map(ele => ele._source);
        this.currentNode.total = rst.data.total;
        this.pageSize = pageSize;
        this.currentPage = 1; // 重置到第一页
      })
    },
    handleExport() {
      this.$message.success('导出数据功能')
      // 实际项目中这里可以调用导出API
    },
    updateTableHeight() {
      // 可以根据需要动态计算高度
      this.tableHeight = 'calc(100vh - 380px)' // 示例值，可根据实际布局调整
    }
  },
  mounted() {
    this.updateTableHeight()
    window.addEventListener('resize', this.updateTableHeight)
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.updateTableHeight)
  }
}
</script>
<style scoped>
.data-table-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.table-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.table-wrapper {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.pagination-wrapper {
  flex-shrink: 0;
  padding-top: 15px;
  display: flex;
  justify-content: flex-end;
  border-top: 1px solid #ebeef5;
  margin-top: auto;
}
</style>