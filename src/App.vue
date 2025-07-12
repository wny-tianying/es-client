<template>
  <div class="app-container">
    <!-- 顶部设置栏 -->
    <TopBar @menu-click="handleMenuClick" />

    <div class="main-layout">
      <!-- 左侧可收缩侧边栏 -->
      <div class="sidebar-container" :style="{ width: isCollapse ? '64px' : '250px' }">
        <Sidebar
            :is-collapse="isCollapse"
            @toggle-collapse="toggleCollapse"
            @node-click="handleNodeClick"
        />
      </div>

      <!-- 右侧内容区域 -->
      <div class="content-container">
        <!-- 筛选条件区域 -->
        <div class="filter-area">
          <FilterPanel
              ref="filterPanel"
              :current-node="currentNode"
              @filter-change="handleFilterChange"
          />
        </div>

        <!-- 表格区域 -->
        <div class="table-area">
          <DataTable
              :current-node="currentNode"
              :filters="activeFilters"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import TopBar from './components/TopBar.vue'
import Sidebar from './components/Sidebar.vue'
import FilterPanel from './components/FilterPanel.vue'
import DataTable from './components/DataTable.vue'

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet2", { name: name.value });
}

export default {
  components: { TopBar, Sidebar, FilterPanel, DataTable },
  data() {
    return {
      isCollapse: false,
      currentNode: null,
      activeFilters: []
    }
  },
  methods: {
    toggleCollapse() {
      this.isCollapse = !this.isCollapse
    },
    handleNodeClick(node) {
      this.currentNode = node
      // this.activeFilters = []
    },
    handleMenuClick(menu) {

    },
    handleFilterChange(filters) {
 
      this.activeFilters = filters
    }
  }
}
</script>

<style scoped>
.app-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.sidebar-container {
  background-color: #545c64;
  transition: width 0.3s;
  height: 100%;
  overflow-y: auto;
}

.content-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  height: 100%;
}

.filter-area {
  flex-shrink: 0;
  padding: 15px;
  background: #fff;
  border-bottom: 1px solid #ebeef5;
}

.table-area {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  padding: 0 15px 15px;
}
</style>