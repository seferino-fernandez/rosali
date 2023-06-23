import { createI18n } from "vue-i18n";

const messages = {
  en: {
    events: {
      table: {
        no_results: "No events found.",
        loading: "Loading events...",
      },
    },
    kubeconfig: {
      table: {
        header: "Kubeconfig Contexts",
        name: "Name",
        path: "Path",
        cluster: "Cluster",
        user: "User",
        namespace: "Namespace",
        no_results: "No Kubernetes contexts found.",
        loading: "Loading Kubernetes contexts...",
      },
    },
    kubernetes: {
      meta: {
        name: "Name",
        age: "Age",
        restart: "Restart | Restarts",
        status: "Status",
      },
      pod: "Pod | Pods",
      cluster: "Cluster | Clusters",
      deployment: "Deployment | Deployments",
      replica_set: "Replica Set | Replica Sets",
      namespace: "Namespace | Namespaces",
      log: "Log | Logs"
    },
    cluster_overview: {
      deployments: "Deployments",
      pods: "Pods",
      replica_sets: "Replica Sets",
      recent_events: "Recent Events",
      status: {
        running: "Running",
        failed: "Failed",
      },
    },
    clusterroles: {
      table: {
        no_results: "No Cluster Roles found.",
        loading: "Loading Cluster Roles...",
      },
    },
    clusterrolebindings: {
      table: {
        no_results: "No Cluster Role Bindings found.",
        loading: "Loading Cluster Role Bindings...",
      },
    },
    configmaps: {
      table: {
        no_results: "No Config Maps found.",
        loading: "Loading Config Maps...",
      },
    },
    cronjobs: {
      table: {
        no_results: "No Cron Jobs found.",
        loading: "Loading Cron Jobs...",
      },
    },
    daemonsets: {
      table: {
        no_results: "No Daemon Sets found.",
        loading: "Loading Daemon Sets...",
      },
    },
    endpoints: {
      table: {
        no_results: "No Endpoints found.",
        loading: "Loading Endpoints...",
      },
    },
    ingresses: {
      table: {
        no_results: "No Ingresses found.",
        loading: "Loading Ingresses...",
      },
    },
    ingress_classes: {
      table: {
        no_results: "No Ingress Classes found.",
        loading: "Loading Ingress Classes...",
      },
    },
    jobs: {
      table: {
        no_results: "No Jobs found.",
        loading: "Loading Jobs...",
      },
    },
    namespaces: {
      table: {
        no_results: "No Namespaces found.",
        loading: "Loading Namespaces...",
      },
    },
    networkpolicies: {
      table: {
        no_results: "No Network Policies found.",
        loading: "Loading Network Policies...",
      },
    },
    nodes: {
      table: {
        no_results: "No Nodes found.",
        loading: "Loading Nodes...",
      },
    },
    persistent_volume_claims: {
      table: {
        no_results: "No Persistent Volumn Claims found.",
        loading: "Loading Persistent Volumn Claims...",
      },
    },
    replicasets: {
      table: {
        no_results: "No Replica Sets found.",
        loading: "Loading Replica Sets...",
      },
    },
    replicationcontrollers: {
      table: {
        no_results: "No Replication Controllers found.",
        loading: "Loading Replication Controllers...",
      },
    },
    roles: {
      table: {
        no_results: "No Roles found.",
        loading: "Loading Cluster Roles...",
      },
    },
    rolebindings: {
      table: {
        no_results: "No Role Bindings found.",
        loading: "Loading Cluster Role Bindings...",
      },
    },
    secrets: {
      table: {
        no_results: "No Secrets found.",
        loading: "Loading Secrets...",
      },
    },
    serviceaccounts: {
      table: {
        no_results: "No Service Accounts found.",
        loading: "Loading Service Accounts...",
      },
    },
    services: {
      table: {
        no_results: "No Services found.",
        loading: "Loading Services...",
      },
    },
    statefulsets: {
      table: {
        no_results: "No Stateful Sets found.",
        loading: "Loading Stateful Sets...",
      },
    },
    storageclasses: {
      table: {
        no_results: "No Storage Classes found.",
        loading: "Loading Storage Classes...",
      },
    },
  },
  es: {
    kubeconfig: {
      table: {
        header: "Kubeconfig Contexts",
        name: "Name",
        cluster: "Cluster",
        user: "User",
        namespace: "Namespace",
      },
    },
  },
};

export default createI18n({
  locale: "en",
  fallbackLocale: "en",
  legacy: false,
  globalInjection: true,
  messages,
});
