<template>
    <div class="app-sidebar">
        <PanelMenu class="m-0 border-none surface-ground" v-model:expandedKeys="expandedKeys" :model="items"
            @item-click="navigateTo" />
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

        const expandedKeys = ref({});
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
                            { key: "0_0", label: "Pods", to: currentId.value ? { name: "PodsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_1", label: "Deployments", to: currentId.value ? { name: "DeploymentsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_2", label: "Jobs", to: currentId.value ? { name: "JobsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_3", label: "Cron Jobs", to: currentId.value ? { name: "CronJobsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_4", label: "Daemon Sets", to: currentId.value ? { name: "DaemonSetsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_5", label: "Replica Sets", to: currentId.value ? { name: "ReplicaSetsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_6", label: "Stateful Sets", to: currentId.value ? { name: "StatefulSetsOverview", params: { id: currentId.value } } : homePage },
                            { key: "0_7", label: "Replication Controllers", to: currentId.value ? { name: "ReplicationControllersOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "1",
                        label: "Networking",
                        items: [
                            { key: "1_0", label: "Services", to: currentId.value ? { name: "ServicesOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_1", label: "Endpoints", to: currentId.value ? { name: "EndpointsOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_2", label: "Ingresses", to: currentId.value ? { name: "IngressesOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_3", label: "Ingress Classes", to: currentId.value ? { name: "IngressClassesOverview", params: { id: currentId.value } } : homePage },
                            { key: "1_4", label: "Network Policies", to: currentId.value ? { name: "NetworkPoliciesOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "2",
                        label: "Config and Storage",
                        items: [
                            { key: "2_0", label: "Config Maps", to: currentId.value ? { name: "ConfigMapsOverview", params: { id: currentId.value } } : homePage },
                            { key: "2_1", label: "Secrets", to: currentId.value ? { name: "SecretsOverview", params: { id: currentId.value } } : homePage },
                            { key: "2_2", label: "Persistent Volumn Claims", to: currentId.value ? { name: "PersistentVolumnClaimsOverview", params: { id: currentId.value } } : homePage },
                            { key: "2_3", label: "Storage Classes", to: currentId.value ? { name: "StorageClassesOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "3",
                        label: "Cluster",
                        items: [
                            { key: "3_0", label: "Overview", to: currentId.value ? { name: "ClusterOverview", params: { id: currentId.value } } : homePage },
                            { key: "3_1", label: "Namespaces", to: currentId.value ? { name: "NamespacesOverview", params: { id: currentId.value } } : homePage },
                            { key: "3_2", label: "Nodes", to: currentId.value ? { name: "NodesOverview", params: { id: currentId.value } } : homePage },
                            { key: "3_3", label: "Service Accounts", to: currentId.value ? { name: "ServiceAccountsOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        key: "4",
                        label: "Access Control",
                        items: [
                            { key: "4_0", label: "Cluster Roles", to: currentId.value ? { name: "ClusterRolesOverview", params: { id: currentId.value } } : homePage },
                            { key: "4_1", label: "Cluster Role Bindings", to: currentId.value ? { name: "ClusterRoleBindingsOverview", params: { id: currentId.value } } : homePage },
                            { key: "4_2", label: "Roles", to: currentId.value ? { name: "RolesOverview", params: { id: currentId.value } } : homePage },
                            { key: "4_3", label: "Role Bindings", to: currentId.value ? { name: "RoleBindingsOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                ];
            },
            { immediate: true }
        );

        function navigateTo(item) {
            console.error(item);
            if (item.to) {
                router.push(item.to);
            }
        }
        return {
            visible: false,
            items: items,
            navigateTo,
            expandedKeys,
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
    height: 100vh;
    z-index: 4;
    top: 4rem;
    height: calc(100vh - 4rem);
    overflow-y: auto;
    user-select: none;
}
</style>