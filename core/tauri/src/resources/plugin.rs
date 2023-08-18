use crate::{
  command,
  plugin::{Builder, TauriPlugin},
  AppHandle, Runtime,
};

use super::ResourceId;

#[command(root = "crate")]
fn close<R: Runtime>(app: AppHandle<R>, rid: ResourceId) -> crate::Result<()> {
  app.manager.resources_table().close(rid)
}

pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("resources")
    .invoke_handler(crate::generate_handler![close])
    .build()
}
