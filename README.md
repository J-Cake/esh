# ESH - The Erika Shell

## What is ESH?

ESH is fundamentally a shell, but it is also a scripting language, however it is very much unlike traditional shells, in
that it
focuses heavily on data processing and manipulation. Programs emit data in a very well-defined format, analogous to
JSON. ESH then provides tools to manipulate this data in a very powerful way, quite similar to how JQ works.

## Why ESH?

Across the board, Erika focuses heavily on correctness, simplicity, performance and customisability. ESH extends this
principle by providing an equally extensible shell.

# Syntax

Here are a few examples of ESH at work

1.  ### List the contents of a directory
    ```
    esh(file:/home/user)> readdir(file: 'file:/home/user') | keys
    ```
    > ``` python
    > { 'Applications', 'config', ... }
    > ```

2.  ### Apply a filter to an output of a function
    ```
    esh(file:/home/user)> read_dir(file: 'file:/home/user') | filter(i -> i.mtime > Date('10th of January 2019') || i.directory)
    ```
    > ```python
    > {
    >   {
    >     directory: true,
    >     file: false,
    >     symlink: false,
    >     mtime: Date('07.02.2018, 7:15:23'),
    >     ctime: Date('07.02.2018, 7:15:23'),
    >     atime: Date('07.02.2018, 7:15:23'),
    >     size: 0,
    >     owner: 'user',
    >     group: 'user',
    >     permissions: 0o750,
    >     executable: true,
    >     readable: true,
    >     writable: true,
    >     hidden: false,
    >     path: 'file:/home/user/Applications'
    >     name: 'Applications'
    >   },
    >   ...
    > }
    > ```
3.  ### Read a file by using the result of one function as the argument to another
    ```
    esh(file:/home/user)> read_file(locate_file('config/esh.toml') | .0.path)
    ```

    > ```toml
    > # ESH Configuration File
    > 
    > # This is the configuration file for ESH. It is written in TOML, and is used to configure the shell.
    > prompt: 'esh({PWD})> '
    > login_dir: 'file:/home/{USER}'
    > ```
4. ### Get the content-type header from a HTTP request
    ```
   esh(file:/home/user)> http(url: 'https://api.example.com/v1/users') | print(HEADERS.i'content-type') 
   ```

   > ```python
   > 'application/json'
   > ```
5.  ### Add a new field to each item in the response
    ```
    esh(file:/home/user)> http(url: 'https://api.example.com/v1/users') | json | .users | map(user -> user + { age: Date(user.dob).elapsed().years } - { id: r'.*' })
    ```

    > ```python
    > {
    >   {
    >     name: 'John Doe',
    >     id: 0,
    >     age: 25,
    >     dob: Date('1994-01-01 00:00:00')
    >    },
    >    {
    >      name: 'Jane Doe',
    >      id: 1,
    >      age: 23,
    >      dob: Date('1996-01-01 00:00:00')
    >    }
    > }
    > ```
