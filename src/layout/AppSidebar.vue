<template>
    <div class="app-sidebar">
        <!-- <PanelMenu class="m-0 border-none surface-ground" v-model:expandedKeys="expandedKeys" :model="items"
            @item-click="navigateTo" /> -->

        <PanelMenu :model="items" class="m-0 border-none surface-ground">
            <template #item="{ item }">
                <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
                    <a v-ripple class="flex align-items-center cursor-pointer text-color px-3 py-2" :href="href"
                        @click="navigate">
                        <span :class="item.icon" />
                        <span class="ml-2 text-color">{{ item.label }}</span>
                    </a>
                </router-link>
                <a v-else v-ripple class="flex align-items-center cursor-pointer text-color px-3 py-2" :href="item.url"
                    :target="item.target">
                    <span :class="item.icon" />
                    <span class="ml-2">{{ item.label }}</span>
                    <span v-if="item.items" class="pi pi-angle-down text-primary ml-auto" />
                </a>
            </template>
        </PanelMenu>
    </div>
</template>
  
<script>
import { computed, onMounted, watch, ref } from "vue";
import { useRouter } from "vue-router";
import PanelMenu from "primevue/panelmenu";


export default {
    components: {
        PanelMenu,
    },
    setup() {
        const router = useRouter();

        const currentId = ref("");
        const items = ref([]);

        const homePage = { name: "KubeconfigContexts", params: {} };

        watch(
            () => router.currentRoute.value,
            () => {
                currentId.value = router.currentRoute.value.name !== "KubeconfigContexts" ? router.currentRoute.value.params.id : "";
                items.value = [
                    {
                        key: "0",
                        label: "Workloads",
                        items: [
                            { key: "0_0", label: "Pods", route: currentId.value ? { name: "PodsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_1", label: "Deployments", route: currentId.value ? { name: "DeploymentsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_2", label: "Jobs", route: currentId.value ? { name: "JobsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_3", label: "Cron Jobs", route: currentId.value ? { name: "CronJobsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_4", label: "Daemon Sets", route: currentId.value ? { name: "DaemonSetsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_5", label: "Replica Sets", route: currentId.value ? { name: "ReplicaSetsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_6", label: "Stateful Sets", route: currentId.value ? { name: "StatefulSetsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_7", label: "Replication Controllers", route: currentId.value ? { name: "ReplicationControllersOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "1",
                        label: "Networking",
                        items: [
                            { key: "1_0", label: "Services", route: currentId.value ? { name: "ServicesOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_1", label: "Endpoints", route: currentId.value ? { name: "EndpointsOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_2", label: "Ingresses", route: currentId.value ? { name: "IngressesOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_3", label: "Ingress Classes", route: currentId.value ? { name: "IngressClassesOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_4", label: "Network Policies", route: currentId.value ? { name: "NetworkPoliciesOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "2",
                        label: "Config and Storage",
                        items: [
                            { key: "2_0", label: "Config Maps", route: currentId.value ? { name: "ConfigMapsOverview", params: { id: currentId.value } } : homePage },
                            { key: "2_1", label: "Secrets", route: currentId.value ? { name: "SecretsOverview", params: { id: currentId.value } } : homePage },
                            { key: "2_2", label: "Persistent Volumn Claims", route: currentId.value ? { name: "PersistentVolumnClaimsOverview", params: { id: currentId.value } } : homePage },
                            { key: "2_3", label: "Storage Classes", route: currentId.value ? { name: "StorageClassesOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "3",
                        label: "Cluster",
                        items: [
                            { key: "3_0", label: "Overview", route: currentId.value ? { name: "ClusterOverview", params: { id: currentId.value } } : homePage },
                            { key: "3_1", label: "Namespaces", route: currentId.value ? { name: "NamespacesOverview", params: { id: currentId.value } } : homePage },
                            { key: "3_2", label: "Nodes", route: currentId.value ? { name: "NodesOverview", params: { id: currentId.value } } : homePage },
                            { key: "3_3", label: "Service Accounts", route: currentId.value ? { name: "ServiceAccountsOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "4",
                        label: "Access Control",
                        items: [
                            { key: "4_0", label: "Cluster Roles", route: currentId.value ? { name: "ClusterRolesOverview", params: { id: currentId.value } } : homePage },
                            { key: "4_1", label: "Cluster Role Bindings", route: currentId.value ? { name: "ClusterRoleBindingsOverview", params: { id: currentId.value } } : homePage },
                            { key: "4_2", label: "Roles", route: currentId.value ? { name: "RolesOverview", params: { id: currentId.value } } : homePage },
                            { key: "4_3", label: "Role Bindings", route: currentId.value ? { name: "RoleBindingsOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                ];
            },
            { immediate: true }
        );

        // function navigateTo(item) {
        //     console.log(item);
        //     if (item.to) {
        //         router.push(item.to);
        //     }
        // }
        return {
            visible: false,
            items: items,
        };
    },
};
</script>
  
<style scoped>
.app-sidebar {
    display: flex;
    background-color: var(--surface-ground);
    position: fixed;
    left: 0;
    width: 190px;
    z-index: 4;
    top: 4rem;
    height: calc(100vh - 4rem);
    overflow-y: auto;
    user-select: none;
}
</style>