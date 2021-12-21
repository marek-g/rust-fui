Fix lifetime of ViewModels:

- Window should be owner of its main view model, the main view model will be an owner of other view models (cascade)

- all the other references to view models (callbacks etc.) should be weak
