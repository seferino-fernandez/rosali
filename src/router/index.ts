import { createRouter, createMemoryHistory } from "vue-router";
import KubeconfigContexts from "@/views/app/KubeconfigContexts.vue";
import ClusterOverview from "@/views/cluster/ClusterOverview.vue";
import ClusterRolesOverview from "@/views/access_control/ClusterRolesOverview.vue";
import ClusterRoleBindingsOverview from "@/views/access_control/ClusterRoleBindingsOverview.vue";
import ConfigMapsOverview from "@/views/config_storage/ConfigMapsOverview.vue";
import CronJobsOverview from "@/views/workloads/CronJobsOverview.vue";
import DeploymentsOverview from "@/views/workloads/DeploymentsOverview.vue";
import DaemonSetsOverview from "@/views/workloads/DaemonSetsOverview.vue";
import EndpointsOverview from "@/views/networking/EndpointsOverview.vue";
import IngressesOverview from "@/views/networking/IngressesOverview.vue";
import IngressClassesOverview from "@/views/networking/IngressClassesOverview.vue";
import JobsOverview from "@/views/workloads/JobsOverview.vue";
import NamespacesOverview from "@/views/cluster/NamespacesOverview.vue";
import NetworkPoliciesOverview from "@/views/networking/NetworkPoliciesOverview.vue";
import NodesOverview from "@/views/cluster/NodesOverview.vue";
import PersistentVolumnClaimsOverview from "@/views/config_storage/PersistentVolumnClaimsOverview.vue";
import PodsOverview from "@/views/workloads/PodsOverview.vue";
import PodLogsView from "@/views/stream_logs/PodLogsView.vue";
import ReplicaSetsOverview from "@/views/workloads/ReplicaSetsOverview.vue";
import ReplicationControllersOverview from "@/views/workloads/ReplicationControllersOverview.vue";
import RolesOverview from "@/views/access_control/RolesOverview.vue";
import RoleBindingsOverview from "@/views/access_control/RoleBindingsOverview.vue";
import SecretsOverview from "@/views/config_storage/SecretsOverview.vue";
import ServiceAccountsOverview from "@/views/cluster/ServiceAccountsOverview.vue";
import ServicesOverview from "@/views/networking/ServicesOverview.vue";
import StatefulSetsOverview from "@/views/workloads/StatefulSetsOverview.vue";
import StorageClassesOverview from "@/views/config_storage/StorageClassesOverview.vue";

const routes = [
  {
    path: "/",
    component: KubeconfigContexts,
    name: "KubeconfigContexts",
  },
  {
    path: "/configmaps-overview/:id",
    name: "ConfigMapsOverview",
    component: ConfigMapsOverview,
  },
  {
    path: "/cluster-overview/:id",
    name: "ClusterOverview",
    component: ClusterOverview,
  },
  {
    path: "/clusterroles-overview/:id",
    name: "ClusterRolesOverview",
    component: ClusterRolesOverview,
  },
  {
    path: "/clusterrolebindings-overview/:id",
    name: "ClusterRoleBindingsOverview",
    component: ClusterRoleBindingsOverview,
  },
  {
    path: "/cronjobs-overview/:id",
    name: "CronJobsOverview",
    component: CronJobsOverview,
  },
  {
    path: "/pods-overview/:id",
    name: "PodsOverview",
    component: PodsOverview,
  },
  {
    path: "/cluster/:id/namespaces/:podNamespace/pod/:podName/logs",
    name: "PodLogsView",
    component: PodLogsView,
  },
  {
    path: "/deployments-overview/:id",
    name: "DeploymentsOverview",
    component: DeploymentsOverview,
  },
  {
    path: "/daemonsets-overview/:id",
    name: "DaemonSetsOverview",
    component: DaemonSetsOverview,
  },
  {
    path: "/endpoints-overview/:id",
    name: "EndpointsOverview",
    component: EndpointsOverview,
  },
  {
    path: "/ingresses-overview/:id",
    name: "IngressesOverview",
    component: IngressesOverview,
  },
  {
    path: "/ingress-classes-overview/:id",
    name: "IngressClassesOverview",
    component: IngressClassesOverview,
  },
  {
    path: "/jobs-overview/:id",
    name: "JobsOverview",
    component: JobsOverview,
  },
  {
    path: "/namespaces-overview/:id",
    name: "NamespacesOverview",
    component: NamespacesOverview,
  },
  {
    path: "/networkpolicies-overview/:id",
    name: "NetworkPoliciesOverview",
    component: NetworkPoliciesOverview,
  },
  {
    path: "/nodes-overview/:id",
    name: "NodesOverview",
    component: NodesOverview,
  },
  {
    path: "/persistent-volumn-claims-overview/:id",
    name: "PersistentVolumnClaimsOverview",
    component: PersistentVolumnClaimsOverview,
  },
  {
    path: "/replicasets-overview/:id",
    name: "ReplicaSetsOverview",
    component: ReplicaSetsOverview,
  },
  {
    path: "/replicationcontrollers-overview/:id",
    name: "ReplicationControllersOverview",
    component: ReplicationControllersOverview,
  },
  {
    path: "/roles-overview/:id",
    name: "RolesOverview",
    component: RolesOverview,
  },
  {
    path: "/rolebindings-overview/:id",
    name: "RoleBindingsOverview",
    component: RoleBindingsOverview,
  },
  {
    path: "/secrets-overview/:id",
    name: "SecretsOverview",
    component: SecretsOverview,
  },
  {
    path: "/serviceaccounts-overview/:id",
    name: "ServiceAccountsOverview",
    component: ServiceAccountsOverview,
  },
  {
    path: "/services-overview/:id",
    name: "ServicesOverview",
    component: ServicesOverview,
  },
  {
    path: "/statefulsets-overview/:id",
    name: "StatefulSetsOverview",
    component: StatefulSetsOverview,
  },
  {
    path: "/storage-classes-overview/:id",
    name: "StorageClassesOverview",
    component: StorageClassesOverview,
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  if (to.name !== "KubeconfigContexts" && !to.params.id) {
    next({ name: "KubeconfigContexts" });
  } else {
    next();
  }
});

export default router;
