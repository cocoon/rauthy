use hiqlite::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use rauthy_models::database::DB;
use rauthy_models::entity::logos::{Logo, LogoRes, LogoType};
use rauthy_models::entity::password::PasswordPolicy;
use tracing::warn;

// TODO remove with v0.29.0
const RAUTHY_DEFAULT_SVG: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="100%" height="100%" viewBox="0 0 512 138" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
    <g transform="matrix(1,0,0,1,0,-11)">
        <g transform="matrix(1,0,0,1,0,-176)">
            <g transform="matrix(0.920325,0,0,1.84151,45.9279,26.459)">
                <rect x="27.741" y="151.57" width="200.517" height="10.148" style="fill:rgb(4,7,11);"/>
            </g>
            <g transform="matrix(1.93472,0,0,1.82732,8.35618,28.7533)">
                <rect x="33.307" y="97.15" width="94.693" height="54.42" style="fill:rgb(4,7,11);stroke:rgb(4,7,11);stroke-width:1.06px;"/>
            </g>
            <g transform="matrix(1.82732,0,0,1.82732,-160.822,70.1806)">
                <g transform="matrix(72,0,0,72,227.174,123.417)">
                </g>
                <text x="128.982px" y="123.417px" style="font-family:'Calibri-Bold', 'Calibri', sans-serif;font-weight:700;font-size:72px;fill:white;">r<tspan x="152.994px 188.537px " y="123.417px 123.417px ">au</tspan></text>
            </g>
            <g transform="matrix(1,0,0,1.01617,-1.42109e-14,-5.24492)">
                <path d="M440.936,322.643L439.204,324.266L255.482,324.266L255.482,305.721L440.936,305.721L440.936,322.643Z" style="fill:url(#_Linear1);"/>
            </g>
            <g transform="matrix(0.920191,0,0,1.84121,46.2464,-91.3383)">
                <rect x="27.741" y="151.57" width="200.517" height="10.148" style="fill:url(#_Linear2);"/>
            </g>
            <g transform="matrix(1.97598,0,0,1.84619,190.187,26.062)">
                <rect x="33.307" y="97.15" width="94.693" height="54.42" style="fill:rgb(43,65,107);"/>
            </g>
            <path d="M439.204,187.734L440.557,189.007L440.557,206.279L256,206.279L256,187.734L439.204,187.734Z" style="fill:rgb(43,65,107);"/>
            <g transform="matrix(1.82732,0,0,1.82732,-154.661,70.1806)">
                <g transform="matrix(72,0,0,72,323.045,123.417)">
                </g>
                <text x="226.646px" y="123.417px" style="font-family:'Calibri-Bold', 'Calibri', sans-serif;font-weight:700;font-size:72px;fill:white;">th<tspan x="288.943px " y="123.417px ">y</tspan></text>
            </g>
            <g transform="matrix(2,0,0,2,0,0)">
                <path d="M219.602,93.867L256,128L219.602,162.133L219.602,93.867Z" style="fill:rgb(43,65,107);"/>
            </g>
            <g transform="matrix(2,0,0,1.95739,0,3.99997)">
                <path d="M36.398,93.867L0,93.867L35.908,128.524L0,163.619L36.398,163.619" style="fill:rgb(4,7,11);"/>
            </g>
        </g>
    </g>
    <defs>
        <linearGradient id="_Linear1" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(185.454,0,0,18.5443,255.482,314.994)"><stop offset="0" style="stop-color:rgb(4,7,11);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(43,65,107);stop-opacity:1"/></linearGradient>
        <linearGradient id="_Linear2" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(200.517,0,0,10.1483,27.7414,156.645)"><stop offset="0" style="stop-color:rgb(4,7,11);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(43,65,107);stop-opacity:1"/></linearGradient>
    </defs>
</svg>"#;

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), ErrorResponse> {
    // TODO remove this block check after `0.28`.
    // 0.27.0 had a bug that could have inserted NULL for password policy on update.
    if is_hiqlite() {
        let mut row = DB::client()
            .query_raw_one(
                "SELECT data FROM config WHERE id = 'password_policy'",
                params!(),
            )
            .await?;
        if let Err(err) = row.try_get::<Vec<u8>>("data") {
            warn!(
                r#"

Error looking up PasswordPolicy - this is most probably a known 0.27.0 bug.
Inserting default Policy to fix it.
You should visit the Admin UI -> Config -> Password Policy and adjust it to your needs.

Error: {}
"#,
                err
            );
            PasswordPolicy {
                length_min: 14,
                length_max: 128,
                include_lower_case: Some(1),
                include_upper_case: Some(1),
                include_digits: Some(1),
                include_special: Some(1),
                valid_days: Some(180),
                not_recently_used: Some(3),
            }
            .save()
            .await
            .expect("Cannot fix default PasswordPolicy issue");
        }
    }

    // TODO remove this block with `0.29`
    if is_hiqlite() {
        DB::client()
            .execute(
                "UPDATE users SET language = 'zhhans' WHERE language = 'zh-Hans';",
                params!(),
            )
            .await?;
    }

    // Delete the old Rauthy logo, if it is the default
    if let Ok(logo) = Logo::find("rauthy", LogoRes::Small, &LogoType::Client).await {
        if logo.data == RAUTHY_DEFAULT_SVG.as_bytes() {
            warn!("Found old Rauthy default logo - cleaning up to use the new one");
            Logo::delete("rauthy", &LogoType::Client).await?;
        }
    }

    Ok(())
}
