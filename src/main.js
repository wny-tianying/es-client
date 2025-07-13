import { createApp } from "vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
// 导入 Font Awesome 核心库
import { library } from '@fortawesome/fontawesome-svg-core'

// 导入所需的图标
import { 
  faFolder, 
  faServer, 
  faUsers, 
  faBox, 
  faReceipt, 
  faChartLine,
  faCode,
  faFileAlt,
  faDatabase,
  faCompress,
  faExpand,
  faEye,
  faMousePointer,
  faPaintBrush,
  faRocket,
  faPlug,
  faLaptop
} from '@fortawesome/free-solid-svg-icons'

// 导入 Font Awesome 组件
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

// 将图标添加到库中
library.add(
  faFolder, 
  faServer, 
  faUsers, 
  faBox, 
  faReceipt, 
  faChartLine,
  faCode,
  faFileAlt,
  faDatabase,
  faCompress,
  faExpand,
  faEye,
  faMousePointer,
  faPaintBrush,
  faRocket,
  faPlug,
  faLaptop
)


const app = createApp(App)
app.use(ElementPlus)
app.component('FontAwesomeIcon', FontAwesomeIcon)
app.mount("#app");
