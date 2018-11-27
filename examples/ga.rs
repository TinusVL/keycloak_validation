extern crate keycloak_validation;
extern crate simple_server;

const AUTHORIZATION: &str = "Authorization";

fn main() {
    let server = simple_server::Server::new(|request, mut response| {
        let authorization_header = request.headers().get(AUTHORIZATION);
        match authorization_header {
            Some(authorization_header) => {
                let authentication_result = keycloak_validation::verify(
                    authorization_header.to_str().unwrap(),
                    "https://login-dev.scoutsengidsenvlaanderen.be",
                    "scouts",
                    std::time::Duration::from_secs(3),
                );

                match authentication_result {
                    Result::Ok(authentication) => Ok(response
                        .header("Content-Type", "application/json".as_bytes())
                        .body(format!("{}", authentication.user_info).as_bytes().to_vec()).unwrap()),
                    Result::Err(err) => Ok(response.body(err.as_bytes().to_vec()).unwrap()),
                }
            }
            None => Ok(response
                .header("Content-Type", "text/html".as_bytes())
                .body(APITEST.as_bytes().to_vec()).unwrap()),
        }
    });

    server.listen("localhost", "8000");
}

const APITEST: &str = r#"<!DOCTYPE html>
<html lang="nl">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Keycloak validation</title>
    <style>
      .long {
        font-size: 0.8rem;
        overflow: hidden;
        word-break: break-all;
        font-size: xx-small;
      }
      #result {
        height: 1000px;
        width: 100%;
        border: 0;
      }
    </style>
  </head>
  <body>
    <label>Access token</label>
    <div id="access_token" class="long">Not available yet</div>

    <label>Response status</label>
    <div><input id="status" disabled="disabled" value="No request sent"></div>

    <label>Response content</label>
    <iframe id="result" scrolling="no" src='data:text/plain,No request sent'></iframe>

    <script src="https://ga-staging.scoutsengidsenvlaanderen.be/groepsadmin/client/bower_components/jquery/jquery.js"></script>
    <script src="https://ga-staging.scoutsengidsenvlaanderen.be/groepsadmin/client/bower_components/keycloak/dist/keycloak.js"></script>
    <script src="https://ga-staging.scoutsengidsenvlaanderen.be/groepsadmin/client/js/keycloak-config.js"></script>
    <script>
      function processResult(jqXHR, statusText, data) {

        var contentType = jqXHR.getResponseHeader('Content-Type') || "(Empty response?)";

        $('#status').val(jqXHR.status + ' - ' + statusText);

        var content = jqXHR.responseText || '"(response missing)"';

        if (contentType === 'application/json') {
          try {
            content = JSON.stringify(JSON.parse(content), undefined, 2);
          } catch (e) {
            // invalid syntax? Display as-is
          }
        }

        document.getElementById('result').src = 'data:' + contentType + ',' + encodeURIComponent(content);
      }

      var keycloak;

      $(document).ready(function () {
        var clientConfig = getClient();
        clientConfig.clientId = 'groepsadmin-localhost-8000-client';
        keycloak = Keycloak(clientConfig);
        keycloak.init({
          onLoad: 'login-required',
          checkLoginIframe: true
        });

        var sent = false;

        setInterval(function() {
          if (keycloak) {
            $('#access_token').text(keycloak.token);
            keycloak.updateToken();

            if (!sent && keycloak.token) {
              sent = true;
              $('#status').html('No result yet...');
              $('#result').get()[0].src = 'data:text/plain,No result yet...';
              $.ajax(
                'http://localhost:8000',
                {
                  method: 'GET',
                  beforeSend: function (xhr) {
                    xhr.setRequestHeader("Authorization", "Bearer " +  keycloak.token);
                  },
                  error: function (jqXHR, textStatus, errorThrown) {
                    processResult(jqXHR, errorThrown || textStatus, null);
                  },
                  success: function (data, textStatus, jqXHR) {
                    processResult(jqXHR, textStatus, data);
                  }
                }
              );
            }

          }
        }, 100);
      });
    </script>
  </body>
</html>
"#;
