# `libseccomp` Version

I remove the build.rs here because it prevents us from doing local development since none of the notify stuff exists.
There are substantial checks done at runtime to verify we are permitted to call those apis.

