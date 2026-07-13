project(
  'application-id',
  'c',
  version: files('VERSION'),
  meson_version: '>=1.10.1',
  default_options: [
    'c_std=c23',
    'warning_level=3',
    'werror=true',
  ],
)

application_name = 'Application Name'

subdir('include')

executable(
  meson.project_name(),
  [
    'src/main.c',
	identity_header
  ],
  include_directories: application_include,
  install: true,
)