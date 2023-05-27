use k8s_openapi::{
  apimachinery::pkg::apis::meta::v1::Time,
  chrono::{DateTime, Duration, Utc},
};

use kube::{Resource, ResourceExt};

pub fn remove_managed_fields<K: Resource>(mut obj: K) -> K {
  obj.managed_fields_mut().clear();
  obj
}

pub static UNKNOWN: &str = "Unknown";
pub static NOT_AVAILABLE: &str = "N/A";
pub static NONE: &str = "None";

pub fn to_age(timestamp: Option<&Time>, against: DateTime<Utc>) -> String {
  match timestamp {
    Some(time) => {
      let time = time.0;
      let duration = against.signed_duration_since(time);

      duration_to_age(duration, false)
    }
    None => String::default(),
  }
}

pub fn to_age_secs(timestamp: Option<&Time>, against: DateTime<Utc>) -> String {
  match timestamp {
    Some(time) => {
      let time = time.0;
      let duration = against.signed_duration_since(time);

      duration_to_age(duration, true)
    }
    None => String::default(),
  }
}

pub fn duration_to_age(duration: Duration, with_secs: bool) -> String {
  let mut out = String::new();
  if duration.num_weeks() != 0 {
    out.push_str(format!("{}w", duration.num_weeks()).as_str());
  }
  let days = duration.num_days() - (duration.num_weeks() * 7);
  if days != 0 {
    out.push_str(format!("{}d", days).as_str());
  }
  let hrs = duration.num_hours() - (duration.num_days() * 24);
  if hrs != 0 {
    out.push_str(format!("{}h", hrs).as_str());
  }
  let mins = duration.num_minutes() - (duration.num_hours() * 60);
  if mins != 0 && days == 0 && duration.num_weeks() == 0 {
    out.push_str(format!("{}m", mins).as_str());
  }
  if with_secs {
    let secs = duration.num_seconds() - (duration.num_minutes() * 60);
    if secs != 0 && hrs == 0 && days == 0 && duration.num_weeks() == 0 {
      out.push_str(format!("{}s", secs).as_str());
    }
  }
  if out.is_empty() && with_secs {
    "0s".into()
  } else if out.is_empty() {
    "0m".into()
  } else {
    out
  }
}
