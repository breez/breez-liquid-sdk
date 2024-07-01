release_tag_name = 'breez_liquid-v0.1.0' # generated; do not edit

# We cannot distribute the XCFramework alongside the library directly,
# so we have to fetch the correct version here.
framework_name = 'breez_sdk_liquid.xcframework'
remote_zip_name = "#{framework_name}.zip"
url = "https://github.com/breez/breez-liquid-sdk/releases/download/#{release_tag_name}/#{remote_zip_name}"
local_zip_name = "#{release_tag_name}.zip"
`
cd Frameworks
rm -rf #{framework_name}

if [ ! -f #{local_zip_name} ]
then
  curl -L #{url} -o #{local_zip_name}
fi

unzip #{local_zip_name}
cd -
`

Pod::Spec.new do |spec|
  spec.name          = 'flutter_breez_liquid'
  spec.version       = '0.1.0'
  spec.license       = { :file => '../LICENSE' }
  spec.homepage      = 'https://breez.technology'
  spec.authors       = { 'Breez' => 'contact@breez.technology' }
  spec.summary       = 'iOS/macOS Flutter bindings for Breez Liquid'

  spec.source              = { :path => '.' }
  spec.source_files        = 'Classes/**/*'
  spec.public_header_files = 'Classes/**/*.h'
  spec.vendored_frameworks = "Frameworks/#{framework_name}"

  spec.ios.deployment_target = '12.0'
  spec.osx.deployment_target = '10.11'
end
