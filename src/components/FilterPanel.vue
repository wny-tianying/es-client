<template>
  <el-card class="filter-card">
    <template #header>
      <div class="card-header">
        <span>筛选条件</span>
        <el-button type="primary" size="small" @click="addFilter">添加条件</el-button>
      </div>
    </template>

    <div class="filter-container-wrapper">
      <div class="filter-container">
        <div v-for="(filter, index) in filters" :key="index" class="filter-item">
          <!-- 关系选择器（放在条件右侧） -->
          <div v-if="index > 0" class="logic-connector">
            <el-select 
              v-model="filters[index].connector" 
              placeholder="请选择" 
              size="small"
              style="width: 80px;"
            >
              <el-option label="且" value="and" />
              <el-option label="或" value="or" />
            </el-select>
          </div>
          
          <el-select 
            v-model="filter.field" 
            placeholder="选择字段" 
            style="width: 150px"
            size="small"
          >
            <el-option
              v-for="field in availableFields"
              :key="field.prop"
              :label="field.label"
              :value="field.prop"
            />
          </el-select>

          <el-select 
            v-model="filter.operator" 
            placeholder="选择操作符" 
            style="width: 120px; margin-left: 10px"
            size="small"
          >
            <el-option label="等于" value="="/>
            <el-option label="大于" value=">" />
            <el-option label="小于" value="<" />
            <el-option label="包含" value="like" />
            <el-option label="不等于" value="!=" />
            <el-option label="为空" value="is null" />
            <el-option label="不为空" value="is not null" />
          </el-select>

          <el-input
            v-model="filter.value"
            placeholder="输入值"
            style="width: 150px; margin-left: 10px"
            size="small"
            :disabled="isValueDisabled(filter.operator)"
          />

          

          <el-button
            type="danger"
            circle
            size="small"
            style="margin-left: 10px"
            @click="removeFilter(index)"
          >
            <el-icon><Close /></el-icon>
          </el-button>
        </div>
      </div>
    </div>

    <div v-if="filters.length > 0" class="filter-actions">
      <el-button type="primary" @click="applyFilters" size="small">应用筛选</el-button>
      <el-button @click="resetFilters" size="small">重置</el-button>
    </div>
  </el-card>
</template>

<script>
import { Close } from '@element-plus/icons-vue';

export default {
  components: {
    Close
  },
  props: {
    currentNode: Object
  },
  data() {
    return {
      filters: [],
      fieldOptions: {
        'sales-month': [
          { value: 'month', label: '月份' },
          { value: 'amount', label: '销售额' },
          { value: 'region', label: '地区' }
        ],
        'product-list': [
          { value: 'name', label: '产品名称' },
          { value: 'category', label: '类别' },
          { value: 'price', label: '价格' }
        ],
        'customer': [
          { value: 'name', label: '客户名称' },
          { value: 'level', label: '客户等级' },
          { value: 'contact', label: '联系人' }
        ]
      }
    }
  },
  computed: {
    availableFields() {
      if (!this.currentNode) return []
      return this.currentNode.columns || []
    }
  },
  methods: {
    // 添加新条件
    addFilter() {
      this.filters.push({
        field: '',
        operator: '=',
        value: '',
        // 默认使用 AND 连接
        connector: 'and'
      })
    },
    
    // 删除条件
    removeFilter(index) {
      this.filters.splice(index, 1);
      
      // 如果删除后只剩下一个条件，移除其连接符
      if (this.filters.length === 1) {
        this.filters[0].connector = null;
      }
    },
    
    // 应用筛选
    applyFilters() {
      // 过滤掉无效的条件
      const validFilters = this.filters.filter(f => {
        // 对于"为空"和"不为空"操作符，不需要值
        if (f.operator === 'is null' || f.operator === 'is not null') {
          return f.field && f.operator;
        }
        return f.field && f.operator && f.value !== '';
      });
      
      this.$emit('filter-change', validFilters);
    },
    
    // 重置筛选
    resetFilters() {
      this.filters = [];
      this.$emit('filter-change', []);
    },
    
    // 检查是否需要禁用值输入框
    isValueDisabled(operator) {
      return ['is null', 'is not null'].includes(operator);
    }
  }
}
</script>

<style scoped>
.filter-card {
  margin-bottom: 5px;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.filter-container-wrapper {
  flex: 1;
  overflow-y: auto;
  max-height: 150px;
  margin-bottom: 5px;
  border: 1px solid #ebeef5;
  border-radius: 4px;
  padding: 10px;
}

.filter-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.filter-item {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  background: #f8f9fa;
  padding: 8px 12px;
  border-radius: 4px;
  border-left: 3px solid #409eff;
  position: relative;
}

/* 第一个条件的连接符样式 */
.filter-item:first-child .logic-connector {
  margin-left: 10px;
  position: relative;
}

/* 其他条件的连接符样式 */
.filter-item > .logic-connector:first-child {
  position: relative;
  margin-right: 10px;
}

.filter-item > .logic-connector:first-child::before {
  content: "";
  position: absolute;
  top: 50%;
  left: -15px;
  width: 10px;
  height: 1px;
  background: #dcdfe6;
}

.filter-actions {
  margin-top: 15px;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding-top: 15px;
  border-top: 1px solid #ebeef5;
}
</style>