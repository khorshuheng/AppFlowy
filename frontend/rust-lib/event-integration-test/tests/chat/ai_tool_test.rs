use event_integration_test::user_event::user_localhost_af_cloud;
use event_integration_test::EventIntegrationTest;
use flowy_chat::entities::CompletionTypePB;

use std::time::Duration;

#[tokio::test]
async fn af_cloud_complete_text_test() {
  user_localhost_af_cloud().await;
  let test = EventIntegrationTest::new().await;
  test.af_cloud_sign_up_debug().await;
}
