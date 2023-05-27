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
