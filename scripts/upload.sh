S3_BUCKET=smithy-rs-site/examples/itunes
aws s3 sync ./pkg s3://$S3_BUCKET/ \
  --cache-control max-age=0,no-cache --delete --acl public-read

WASM_FILE=$(ls pkg/ | grep '.wasm$');
brotli-cli pkg/*.wasm
BROTLI_FILE=$(ls pkg/ | grep wasm.br);
mv pkg/$BROTLI_FILE pkg/$WASM_FILE

aws s3 cp pkg/*.wasm s3://$S3_BUCKET/ \
  --cache-control max-age=0,no-cache \
  --acl public-read \
  --content-type application/wasm \
  --content-encoding br

aws cloudfront create-invalidation --distribution-id E245EU483WK70N --paths '/*'
