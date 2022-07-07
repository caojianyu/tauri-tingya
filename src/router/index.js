import { createRouter, createWebHashHistory } from 'vue-router'

// 1.导入组件
import Index from '../view/Index.vue'

import Bandstand from '../components/Bandstand.vue'
import RankingList from '../components/RankingList.vue'
import MusicVideo from '../components/MusicVideo.vue'
import Ilike from '../components/Ilike.vue'
import PersonalStation from '../components/PersonalStation.vue'

// 2. 定义一些路由
// 每个路由都需要映射到一个组件。
// 我们后面再讨论嵌套路由。
const routes = [
    {
        path: '/',
        component: Index,
        children: [
            {
                path: '/',
                component: Bandstand,
            },
            {
                path: '/rankingList',
                component: RankingList,
            },
            {
                path: '/musicVideo',
                component: MusicVideo,
            },
            {
                path: '/personalStation',
                component: PersonalStation,
            },
            {
                path: '/ilike',
                component: Ilike,
            }
        ],
    }
]

// 3. 创建路由实例并传递 `routes` 配置
// 你可以在这里输入更多的配置，但我们在这里
// 暂时保持简单
const router = createRouter({
    // 4. 内部提供了 history 模式的实现。为了简单起见，我们在这里使用 hash 模式。
    history: createWebHashHistory(),
    routes, // `routes: routes` 的缩写
})

//确保 _use_ 路由实例使
//整个应用支持路由。



// 导出路由
export default router
