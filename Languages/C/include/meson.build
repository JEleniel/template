application_include = include_directories('.')

identity_configuration = configuration_data()
identity_configuration.set('APPLICATION_ID', meson.project_name())
identity_configuration.set('APPLICATION_NAME', application_name)
identity_configuration.set('VERSION', meson.project_version())

identity_header = configure_file(
	input: 'identity.h.in',
	output: 'identity.h',
	configuration: identity_configuration,
)