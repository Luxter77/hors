# Hors

Hors is a data procecing tool I cobbled together for creating the corpus

## Usage

```Markdown
hors [OPTIONS]

Commit crimes against human kind, just like God intended.

Optional arguments:
    -h,--help             Show this help message and exit
    -v,--verbose          Be verbose and slower.
    -n,--runame RUNAME    Run Name.
    --overwrite           Overwrite existing corpus{es}; Useful when working with
                          limited disk space.
    -a,--arkdir ARKDIR    Directory directory where the EPUBs are stored,
                          directory hierarchy does not matter.
    -o,--output OUTPUT    Directory where to store the resoulting file.
    -u,--uniq             NOT IMPLEMENBTED YET; Filter adjacent matching lines
                          from corpus.
```

## It works on my machine

I wrote this tool to work out of the box in that configuration.
I open source because I thought it may be useful to someone someday maybe perhaps
Also, to brag that I can use rust `:P`

On my machine, I have my things stored in such a way that:

```YAML
project_folder:
    - pony: # This project
        corp:
            - $(runname):
                - *foo-$(timestamp).txt # Default
                - $(runname).txt # If overwirte == True
    - *foo: # Other projects
        ...
    - tools:
        - hors.sh # this is the old hors implementation
        - hors    # ideally, this binary
        - some.r
        - other.py
        - tools.sh
```

And I wrote this tool to work out of the box in that configuration
I also added manual overrides for that but it's kind of painful to use
At the end, is up to you :person_shrugging:

## Original implementation

The original `hors` implementation can be found on [the original folder](original/hors.sh)

It is slow and painful but it does the work, after about a day of runtime or so.

## TODO

- Implement -u feature.
- Add support for working on compressed dump.
- Add unit test (because why not)
- Add option to disable or change prefix and suffix
- Add output formatting:
  - JSON
  - CSV
  - ???

## In the future

Perhaps ading filtering support directly on the program, such as:

```YAML
filters:
    - by story:
        - by tags:
            - characters
            - series
            - warning
        - lenght:
            - spesific
            - flash ( <1k words)
            - short ( <5k words)
            - small ( <10k words )
            - medium ( <10k - 50k words )
            - long ( >50k words)
            - custom word count
        - rating:
        - popularity
            - highly rated stories ( >80% rating ) 
            - poorly rated stories ( <20% rating )
            - custom
        - view count
        - content rating
        - publication state
        - completion status
        - publication date
        - chapter count
    - by author:
        - ammount of stories
        - word count
        - creatin date
```
