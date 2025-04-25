# Changelog

This file contains the changes to the crate since version 0.1.1.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.2.15 (unreleased)

- Added a note to the README that the implementation of Fukushima's method is
 simple enough that if the input is known at compile time the optimizer can
 sometimes evaluate the entire function at compile time as well. The only reason
 for using the word "somtimes" in that sentence is that I haven't tested all inputs.
 Every input I have tried has succeeded. For an example, see
 [this compiler explorer link](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywYgATKUcAZPAZMADl3ACNMYhAAVi4ADlIAB1QFQjsGFzcPbySUtIEAoNCWCKjYhKtMG3ShAiZiAkz3Tx9K6oFa%2BoJCkPDImPjLOoam7Nbh7sDekv7ygEpLVBNiZHYOAHpNgGoAL2QtgFIAZgARXcOvACEtrnXMA40AQSoGLfQAdw1kCD2QLaoAGySOZbAC0x2w/yBhwA7JcHo8tkitngqLt9scnFsNAA6DSw%2BFPZHEqGSEAgYKPYII4kHGFnTC0JQotF7Q5HJzHM5eHHRSQATgA%2BnF%2BUdBfyNADBdENGLJDCxQCCTSSUjtgB1dmc07YnEio6kLYADUFXARKtVxCYtgEYkFVBMDHaDAgFtVyL2OIUAEcGhAFm73YdooTEUH3aDcfzo%2BKY9GhfHYwnSIHw0jQTyFQmAVK4jnhV4NIKjjmU0S0yTcRovFKuEDBYWYYKYTXY0XpKm0wCcUcYUX%2BXXhRomyLJOKNFwy2GK8iYTiuBPi144oKAbFm3FooLJLKpzPiTz%2BXEixo4l5VzCV0cNEWAZvi3v90jcZI4iuuPyt/y76be8Kj82j5PlWXhijC0RirKW6Xk2Rxwauk6duGVY3oKw6wWuzYAieorNi224GkhtLRCcQHhgcIZEe6XA4mRM7RHiMJcAhRZxK%2B25cE2kjLnRFb8jiR65kcK5xPExZ8tKPFUaq/E5kW4Fbh%2B56NsKcQwrxaaSDiML8mOvbniKQpcPBAJcD40kkjRdbnvyfbFvKeHttE56EeWwHzmJApjgCXhbjesF/tE0QacheKygR55ruesRil4O4Nl40UhcRpFIXMbp0gyTKYCy6JalyWySEcOKmWK/JxduPmrgCQq9sqbkkhq%2BU6pmfKGiaXjmg1xJWjazC0PajrOq63Xup6Pp%2BgGo2qhRoZPumUZxpeaFiTe0EiqayURrifZjlw0RNrEX4aGOcQlltqo8mZsWtqesXHg2znChdJI8sJQoAjCUrDkWXi4Qd5nTaFMIg6uEGmqtMoNgOz0WcSVYbXeUo1Su0aKpIY7gS98NhWK15SnFK4HVuvbeR2QNBihJ6%2BaakpYe2Om/gZzkvRRqUU8is1w8iNHY8ixVQau1armeQ5E5IiEc6qxVSHK4MTopy4Xgk3NIjR%2BnFvtYPI9EQpxZjfPPjivkrhjUq2XtB3btxm2q7qE5CkFzHWY9zGiUKJ3BXbVPbmpza68WRznhjJOLjWhtbG9g6xMxd57cOmtbtEmCggCrMkSF6UNZlWyMsyqJ5ZiBW63EOKSODEFgWJl4GfV06NesWyakXOpaWek7GsWXX1z11rpHaDpOn1I092NyBer6BD%2Bunc3zZGAmIzCY6Fnj%2BEKl7UuVryn7Cl9Q7fQCZVFbDm/EjR3HRaexaLqJim6Q%2B3tl%2BxZ4iTDd6xcJtun8iVYY9KJ3imPnBFyEo0KuVHmmFCzFnJu0Zq%2BMUfJmKfhHBHH2E5vr4RrAg6IUoZTKUBhA8MxUNZwVgsLWUt5YgpxVpvNmM87Y0Q0BHGiNUvzxVAkKSQVVTJCnUo/Y8H17wKktn5DQ/JUHznitWIsRwBSxnfHZUCEiby1StvEEccjewezrMo1CoFopSF3l%2BeCIomwtmYXqOmE4mzDkUlDP6/IU5pztjyYmaF8wKjAouXWLkU7iIsnQtKGV6S52yrlNkLczgrhOvxAUSD7y2KwtAuuQYmqRK2HOE6HcTSSG7kGXq/cBqD2GhZcak9p4BMot/Ba2kFSmnrAOBM%2BFJQuQscZIy8EjhQ24h9Oy4D5q6iKs7Uypp8JBXPJxeS5NCGUzxF0vCkFlpHA/GAw6GgN4zPdD7b8Y4JZSh3FuY8SdQH%2BOqVsUutkXL3nLkKPMh01JOIjq4qqXSZGGIlsxEG0VqEWKKqdUSxZQZsVjofU0KcuCS02UiQJtCqlQssniCRS9cH4TUmOHBik/4EIGbiMyBNbKOUDntf6ujzZL1NOuP6X4cGaxofCnGN4ixSFYrhI45VrbYOwRY6s0Fj7LivFrP6elHkuONqAmsK4mLaKQaBPxEd%2BJeDgaBWmzEg77OTqCYy6d2aEKzvXHOeccoFwiRyAqELnZxQEhJMyzEvCcQLCk90aTTU6n4sJJhndoh5PdAU20RShrD1KePCaU8prwq5mc3FhY0JMRWlKJiScSyqQkWdBBrYdwrgcaaJW/T5pVjslIc8IMvyDl0leaZOKwrMVfN5eZOlxaHTMtuXRJ4FzbmWuBDCtY237Q2QM1xsbogbT%2BpKhOC4wI/IYQJT%2B3Fq0OW/LeBykhwWQv7XqP%2Bu1xxlWMoArg4K%2B0VhheGuFAzGESN8mVT6xY3zbhpR%2BW1fDI14ibWdPWn8JQBTeXSytjKGzC3AmBGlQdFJiPFN%2B%2BaVl2JUqFrBP%2BA5l2pyeT2MSZ0bHYWlFe7i%2By5VTu4k2D8nDo7kt8thzV2KIO8hvfA1c98OHto%2BuC5xsKdVBj1bSEJhrwkYhdWcGiQ6bE2yPLaq%2BiVeGYCVHSWeqpnXal4zRAR7VVzestH3P1g0h7pBHjOMpk16FnJoi2CZbaMZlVwiDZ2Ei2VGR3rpAy9Y2IGV0U2I8RMJITjHJKImu5H6/pOgZDaS9lLg18s5pRds5yiW8qLUS0SnpHhhJOs5WkhPilBWpfZ8yuFxBXfK7SmFTyo1jVh6URx90R2Kn2LcWaWwKImfhAE4LTnHpY4ek9FGmE%2BfJf5Ai75VrRqCro5iso8ZQ32reGqeE9oRy0kVb695jwILXlQxDdtuw12hra5Zq4/wLgQ01gZxV34ES3J9eSoLdKOLIxHbsd0ELUu%2Bq2bCCGdFTsi2B828ES7ZqLSnQs2rM7BKyvnVk3HZORx7BLdtrF4rqOlFwzAMJHXScbs3HjNwtLXgSJ3GEymSS%2Bv6upkpm8dOhr0/S/m2k4tbZ3C5Y%2B5c9ISP2mOAccpWwSkW9Ex95Oja/vAsjandNwIewTvt9rir6n9mVbrc2BLssraS9pe8QdMZXt1pBSU4KOtnIi0HIcHnQbVkOvKcFFb5rFV0njcKvZDlstps98DT5ipnnugTfr%2BZqwrgaxmXNaYj0zgjdzm4tEfPRohTZJXn9Ny7Ixi2mNmMm3LlrBJM6TLTdPlLq%2BE8AdjwyIm8JUroIHf7mKhLTNNsJaQT/DVatOW7ZtzzKuGmLPUsex3hq/aSGzIuT/CWSKZ1KoIMaxYnMy8AI4Pw7dIdxZftp/IhnIJ2cONhONaDgqxUgpNiBFxa8Q5%2Bzfm3JgOISOG5N2arx7seZgqdziLj3ufUB4Bs00GieunKlSf3DdlZa5l7rIw5QpjgeVY4MAoyMqKkOOkI4ABP6qEkoek8UbEes4EmsuCne%2B%2BEo2iJ4oG0uzYiWgeri7K9a/4XiluUoBeZk3KoMjSwog48QTKT0CWmqUBFGvY0S5eoGC4nyG0l2dq5WPYe8piiccej0Gqiq/2b%2BU6uiVWD034V4dk3EeMf4s%2B9E845Co2dq4o76OCIq%2BmvIN8W2omCUNi/ee6oIouT4fGV6IM426MqqKG4KPu%2B4DEOkesAC10/4zsb44KReM43YMUmsNiBuuuw4cuYWOhQcRYMo0SO65Uqq8UXuUkzGAOi%2BQORqIOZ%2BWwDEQUTKsaPkH0g4JYpW/Ix%2BxIMmZqsSd4im/It%2ByI%2BOD%2BGmAgWmFYJOFSsK7%2BM4GeSs1YuYUMYihWyi9qva/s4sBMjMTEDhyht2OY54ko5sosooXg2hgeGOmWoCJYRk/eJsB%2BmqWugeNEy48cnyhiOC6KU%2BZBShFY6skOA4t4e8Fs0oN6XuH402yGTKE2bEScciiUBevkSGQIykQCNshYIkH4KcyyYhrREhPmi4XSTYhYNxW4NyJWTKvBXSaacB74NsPkKcXOa6zkt4aBFUfx2apGEK122kO6PkkU4u%2BMaEEEjGZJwC24iU1U0S1yP48Rq6FGokBkg4S8mBqqoM8RTBT4bq%2BYK8aEv%2BokiocucE4JuqgOoSwOhcaOpcdyUOvsl60El4mAC4xRyIpRLUV05BncC41RSItR/q9RLoz%2BIaLRx6bRFxUYW2koH8DMwcR4khCUwcyq9OaEFUQUCCvBzhLE0osa8QQ2xk2JEcpcFCoZqieMkOsojx5GT4DEzetkRMVUfYHsjxOJFGEEma5C8yn4lu3xKZ%2B4exSso4HEVJuYEKv23hFxeo%2BYkR4o4uMxK0vioIXScp/ubW5hweT6v6si90NiNMMc4oSBjZaYV0tuEEdibs4Mb4wqhezx6ywc9qE4nuoK%2B0tU9hSGTEGJwcW2lcoZJhgwq2ZcS5AEOYYoZ0Hx3B5xmkJUciXCApRMN6IonuoJOxAyN2WKwss2zYf0Ghl2vYvZ7obGnMS%2BSpJqYO58DkIM6xuC4uf0MiOpXgepaoKO6RlqOk2SEMZpWwFphOgaxOwa5SYafZDpaYxkuhNkx8p4LKd5TEg2kkpsoKa4t4sijeyMB5WsNU8a8yZ4BMPFGq054Y58EKYCnCv%2BZky8g4Cxmq4xFYzyERdSQy2aSCCoQ%2BU6RJZktU%2BEGM0UJiv2B6juxsuuXklURkx8Q6CGQcFihYCJYp9YZkikSyoJZhc%2BLWvu/ZFZg5gBYUJ4Qi3iL6jMp4nCwpAVcEFq0ei2VWE2Sl3lPhxsy0Iowcf8soRMXSZWU6J0shwJVWNMiU7Yv2v5803YH4cBCY%2B%2BPkzOdYv2eZA5EKdB8aDesoHmMMMpKlaY6%2BAWn8YewoO%2BIopGEsEFqoUF0KMFqRypYOPIHypo5cwooKvkHhycxkWFWwBp3Ic4MoHqJoZkRFJFxSZF8KzRVFrWNFklVVnRe8dY2%2B2i0VM4PsRUio4Um4ny9Yp2gEr2E2QIMifs8Q0x5UOGZyb0E2nEjse8NUqekqeVOh6yby6xkU%2B%2ByyCJv2nJA5QItYBBVsJYNxpBDZEcc4zeZ4m%2BV8siXEzkoJz1FYc48Q6KG0nVgKrxDBMelSvlc%2B11QYvMUJ1MzsEkvkiVYoC4DF0ZKht40lfIy8uuQccupJdssSWsX22Eykpal2nEvBakBiRa4qIFdJ3uEcDEh8uymE5q0oVs6yDB14SGr430O8C4ps6hN4Be4FU6GZw1DMYENsIM8NoI5cE1JIU1sIKRXG6RWkkVzY4UQIy86Gp4uyOpkgW1O1Ww14llacJpRwx1qmBOp1T%2B5FL%2BpO4h%2BmpcOkpBi4C4iauyeYseEKty98b1xY%2B%2BXC3kKVTZcUpBf44Edmm%2BDeZBFVFl7cgCJ282i4soDBWtdeKhSceeuEPKf6IRT5klvIAczkoty1T2woXCWxYJr2WsTEsEQU18Zi0YXlvBR48kMYIMGWkUnEKcr4QdQYAep6gVAyDEdYRk4UH4IkxZoN8uSxeouuG%2BpoossVgK555ZM46sdkl4Ik5K1eNGBkXhFi1ut6c2oimVIR5l%2B49eHhmaBKOCtYPVxtxs64eYe00Wm5dqD9WNAVDsDY0ln4tUO68oJ2D97dM08%2BQMIdBqy%2BaR6SmSx9usqem%2By00YCW%2B0KdOF6SQcAk34imUgOd9%2BlpRO51FFr%2BEJ4NfGqEQ6BkO%2BOYAUy9syvmVlVxfK3d8yiqzV%2B4MZe8IB9ksc/eCk4KUDFYaZz8f8WRCEykIoulOhEE%2Bybapkkq6hQ6QpvyVuyqBl0dv0n8Ntxj7o5uTefsiqm26l7NvV4YPILS44nun8iUWpioKcLMnNZOr9Tl64EEX1y8QCm4ixa6Z0Q21O6hr4nyw4%2B5HtNsJYyMkOpkLkQI5VzxTEdmscqEEs0EJYxN09J0ChmMtu8Qpmid3ZdNtFeIVThiUgekxhdhAdiTqoyWW2QIreCCx8NUkUJT%2BzwYXNxIvDM14d6SexSack/s0EBKn0yDnE0jp%2BsjThOkijXqTwSEJ1j%2BDRNplF5T80lqoEhyTaEEG5ZKqz4YpcpkSeERMYB0fdDT80H9Vux8%2B0xDuYiqnTZyFy9YdGPT9xzOgzGYdDyhzkCJSapetMvC3Ev2nDK9hY6KCciqwcv%2Bkot4oJNj7RcysJ199qUUIsCG8o3KAC8GaEAKdqZMe5oIOCT97oL9FGvBN43k91bE7azOr4UZelO%2B1xTJRhHxJJVzB4JU%2BriU%2BG4U1YDdZZFiOCSCV6WCDYy1b474P5vBdqotP47OMlzYN559oqrtDYtu9VSJWEpsD9ElQYPIvk0x4u5cLlm%2BcEJTibnM3D8pyRips1cFBUV0cYPe7EOkotOpR%2BkmSEqdwklGPgJpAIyjhSpFBd6jRddp1FLi3Y0Y74PFVNYGXE1ryIyblKNMN9/ssJSsCGSLQYMsT0bhiSgkNG8aJuqD0lO4i22lOZLrHtAly027zdoluCoJdLfEeo%2B%2BfYMiv%2BsiNdtDrr9Yfm16tyf8bEcoJTg9uDJU6hPC18Hm%2BYr4DBpk6rXDPN1Eb980DNVsNYmbNioC4EcuObMkZDWet47B0t4E9Jdsh2yqKWn4maHaVsIho7SIDEW5eEtqmiD0fmoJWTSbV5uCDeP9BYbsHTAdbjfVL5CCmEH7zdRkOYJTXH4Y/ES8sUcisBK01dTiDHubNzyIdzYdK%2B6R5uP4EEesKyZ0uyi4DWH43zqOYOEsZcJ0ijOOQLQMILVpjRaYF1kLT4f0iu6KO9fY%2Bj2lpDEBoyGWmn%2BGWboIwrTZEosxsJjMNYTKn4CNuxzZuYdkp4Fru8RNGYZHWw3Y3EWeQ2O8pCaEukoJyXEWCcUxlUfkAKpiD9yXh4ws34d5/1O8h8nylzSG7moDAmhyNKITTiODz9ebPbOhw%2B7KZstKaEO9tkERuB7984fSuEml7lt6z2AXz5MEFKsU%2BEBLEpDB4cU6RD1sERGxEkR0oJnXTZokthYEAEoEA7sUD9h3XYEOCJWsHugCjsbLqr877okdPLLdwszk0eTi83JIfukFCpnGKnjzZcTafYja6KabmECxu2BnuFsiPYQ6hocgoDrbam%2BdYLhdtALol1flEHqoO0f8xkzO982EtqV6KHW8EsHmVlSatBtJtl566GM3QIIKZiPFL2Q5rT3SVtNm6GQqse5QwFYE9WDk6yRkzxMJNl4kQkSCR42LA5oEFdyMkp7KkyOBq5Stz6hGmMMNpr34rjYH/3/l0DUH7kcEp0y0rVF4uryJIegGQkek98B0t4oh/NcOQWXdIVHsoMcnBzZcf4X5EpVWVs34Slr3qoexZzU%2BfIvC%2BYokY380/E1Y0Uos4TWEoteYRvdsAdxsv%2Bl4io7Zbaly/jzWSR%2Bq9ztbQM2wSIqPH46PedoL1p2PuP9n%2B488pkmKe0SsZkYrm%2BEin0vCuEHBgcFdtUyiiMSu4Mm6Q6hyInJj3i9Y1lUgV4AEooVPxIc4wibYzX1BmbXukfCKy4gnmMEkOYCYdlSfT4mSzNwDAEAhwkscOfZy/EwKk5Hs8EeGk5SlXP5fJdUXZRASmvC/RQUTEdsIuA5pDldObaEsCF1OgAUj%2BW/PUBVBBjxxb2ZiRXgFSFRtlawI1AlCT1XDX99wmSG8jDBtQaFl4Z0F/oHjf6YklYMoZeMyV2g4czkmqbeN9EMQthEqGHdhkl2N5KdzQ9IbuAcFAizRAgtAKYBADEDvAmAAATwUDpQSICIF4MRVzp1FnQmINAAwHMBbBggfwMwHgB2CYBDQ2g3QScAMGpBjBEICAOoFJCGgGA7gSINaBIB/BZogIXJEcGuDUgM4bwRgKgBYCBAXBUQYMJcA8HHBrgJwNmCCHBBHBIQHg4/I4LYC9QSAOIQIEQEFCEBIg/oHEMQEwAAA3HIfwFoDoAIAVYQ0HSCcAKB3Apg1AJgCoBUBKhWwaoSwC2AAAqLYKoAuDXA0A9QqgCCEbhYAGAAQoIUQGIBpDBAqATIQQGyFzBchBQooa4FKHlDYQVQmoVsF6ENCmhLQ9oZ0O6EbC6hDQvVJlG7iqCDAMwBoIKE%2BA/A/gHgmIRCFJDH4PgXwH4PsMYTbUDheQ8kB4PJBmCCACgckNgGOHCCLOjwbYEIAQBLAShWwCIBkmrC8hZEakSUEvHSxSotgTAZAGgGID4AjAWwIgAiG2AIACABARIACM2DvAKROId4K4CoBWgWAYgRIAgCYA4g0ALAdYIEESAmACAxwAAGJ4AuQfgVgKUAIDqhRBQUS4BoDFFeBOQVwKUaJAzBijog2AVVtnRTb8g2hnUYWtgDFq5IU2PIzUUFCiFPBtghgdACiAIAogFAcI1xIiL7CfRgO9onEASMbiPALR9AJgLoIIBMiLReAK0e8G9FbA5BSwLYMAEwA%2Bi0QQYkwAcIYD5DIgFor0TlAQCYAuh6Aa0EwBRCvAEx0Y8wIYB9GZikxWwJYCSK5FbB3ggQdAKgHeB4jUA6I/4LQFQB9xcRyQdIVsCSGlAnRTwTkWEH%2BCvA/hgoAoXaGTGsBEg9Af0GCAeEJDq%2B9cc4cKKuEaBqAQIckAAFlHgRoD4Tk2BEnAOACwWgJwGiC8BPAHALQKQFQCcAAASmYAtEKAlgKwHKKIOWS8ACAmgbcQsAADWMQKmABjEQYxDyacXcRwEkAHjnxJ4zgLwABFMInxR47caQDgCwAkArIxIHQEiDkBKACEpCVEGIDxBsIfAOgDMOIAAiIAYQYCWECCHEA5BnAHgKQBInMAyJAAeTCDaAqgkEyiayLYCCBaJDAWgORKgmkAsAYQEwMACcBiAmQFE3gFgHpFGBxAPE/AHkOqCxiARPE5MVUC5FrBKJ6QxkMBMkFhArQZElwFgGAkEBiAeAFgKJNICxjiAYQFIJgBOCYAJJwASQUYGfELAqABgYAAoAABqeATAO8FomJBGApk/gIIBEBiB2AUgGQIIEUAqB1APE3QD4AMCOTTA5gfQHgDCAAjIACwVAIkD6gKTQQWITKQQFBD0BYxtALkEcF4CoBzJRkrAGlKgDMA2AIATAPgD6hmSxAJgNYIWDtTQSIAdU9gPUGQAIAWpbgdgKoAbxAhQQjoV8cMPeAMBCpgQEwKoFBDABHBcwBYIkEMAhBWAawEab03GmTTKxM0hyfNMWmOCtgEARKQQH2DyYBYcwMCYyCYnpAHADAZwK4GaB6B/AUwYoKUD0DJBUgfUUYJ4EnA/T8gDAHoJ9P6CTg2gfUToCMBenZAIZd050NDMmBFA%2BgUQCGRMH%2Bl6AcxDQUGajIkALBrxywVYPjP0B7igJPE08RwG2mrhJAu0qaQdLmkLSlpUYs6ZeMulAM8QN00gJBK0ArTSASYpgFgCiDTx3xAoeRuVC4QShxkt8UmQBPJnHjKZYEkABBKck7jOAXgeWeVNAnczVZZkyIKkHsCSAgAA).

## 1.2.14

- Internal documentation improvements.

## 1.2.13

- Documentation improvements.

## 1.2.12

- Small improvements to the implementation of the complex Lambert W functions.

## 1.2.11

- Documentation improvements.

## 1.2.10

- Fixed a bug that could result in incorrect outputs from the complex `lambert_w`
 and `lambert_wf` functions near 0 on branch 1.
- Added an assertion to the first example in the readme that the computed
 omega constant has the claimed properties.
- Internal code improvements.

## 1.2.9

- Show the import from the `approx` crate in the doc-examples of the functions.
- Test the crate on multiple operating systems in CI.

## 1.2.8

- Internal code structure improvements.
- Internal doc improvements.
- Update transitive dev-dependencies.

## 1.2.7

- Internal code improvements.
- Internal doc improvements.
- Improvements to CI.

## 1.2.6

- Internal code improvements.
- Internal doc improvements.
- Update transitive dev-dependencies.

## 1.2.5

- Remove `libm` as a direct dependency.
 It is only used through the `num-traits` and `num-complex` crates now.
- Updated transitive dev-dependencies.

## 1.2.4

- Improvements to docs.
- Return early in the complex-valued functions if we know we will not be able to
 compute an answer

## 1.2.3

- Improvements to docs.

## 1.2.2

- Improvements to docs.

## 1.2.1

- Internal improvements to the implementation of the complex Lambert W functions.

## 1.2.0

- Added a `f32` version of the complex valued Lambert W function.

## 1.1.0

- Added an implementation that can compute any branch in the whole complex plane.
- Added the `must_use` attribute to all pure functions.
- Added the [`num-traits`](https://crates.io/crates/num-traits) and
 [`num-complex`](https://crates.io/crates/num-complex) crates to dependencies.
- Deprecated the `LambertW` trait, as this crate is not really the place for
 such API decisions to be made on behalf of others.
 It is also unclear how the trait should be defined given the newly introduced
 general implementation.
- Updated transitive dev-dependencies.

## 1.0.20

- Updated transitive dev-dependencies.

## 1.0.19

- Documentation improvements.

## 1.0.18

- Made the error bounds in tests stricter and clearer.
- Made small improvements to the examples in the readme.
- Added a CI job that checks and builds the crate for targets without a standard
 library.
- Added a CI job that locks all dependencies to the oldest possible version according
 to `Cargo.toml` and then checks the crate.
- Added daily CI jobs that test the crate on nightly.
- Added the "no-std" category.
- Updated transitive dev-dependencies.

## 1.0.17

- Clarified information in README.

## 1.0.16

- Mentioned some interesting properties of the method in the readme.

## 1.0.15

- Don't panic!
 Enabled static verification that this crate can not panic using the [`no-panic`](https://crates.io/crates/no_panic)
 crate.
 This does not affect any dependers, as `no-panic` is added as a dev-dependency.
- Added a CI job that uses the above to ensure no panics make it into the crate.
 This verification can also be run manually on a local copy of the crate by
 setting the environment variable `LAMBERT_W_ENSURE_NO_PANICS` to 1 and
 then running `cargo test --profile release-lto`.
- Implemented all the rational functions using a single generic function
  instead of several different hand made ones.
- Sped up the `semver-checks` CI job.
- Removed the "no_std" category from the crate, as it's already in the
 "no_std::no_alloc" category, which is a subset of "no_std".
- The text in the README is now also the crate documentation on docs.rs.
- Added tests of the functions on a large set of valid randomly generated inputs.
- Made the docs.rs badge use a different color and display the crate name
 instead of "passing".

## 1.0.14

- Updated the dev-dependency on `rand` to v0.9.0.
- Added a CI job that compiles the benchmarks.
- Added a CI job that tests the crate on the Rust beta branch.
- Updated transitive dev-dependencies.

## 1.0.13

- Removed the note about the accuracy on the trait functions,
 as that is different depending on the type that the trait is invoked on.
- Updated transitive dev-dependencies.

## 1.0.12

- Noted the accuracy of the functions on the trait in the example.
- Improvements to CI jobs.
- Updated dev-dependencies.

## 1.0.11

- Removed unnecessary import in `integration_tests.rs`.
- Improvements to CI jobs.

## 1.0.10

- Moved unit tests to their own module.
- Corrected some information in code comments.
- Simplified `semver-checks` CI job.
- Made minor changes to the code of the plot example to show the
 function clearer in the final image.
- Note adherence to semver in this log.

## 1.0.9

- Switched the way the crate depends on the standard library such that the
 implicit prelude is always the same.
- Sped up CI runs by using `taiki-e/install-action`.
- Added an example program that plots both branches of the function.

## 1.0.8

- Fixed a bug where the principal branch functions would return NaN when given
 infinite input.

## 1.0.7

- Moved tests to their own file.
- Made the accuracy of the tests clearer.

## 1.0.6

- Added more unit tests that verify and showcase the accuracy of the
 functions also at the branch point.
- Verify the MSRV in CI using `cargo-msrv`.
- Check semver compatibility in CI using `cargo-semver-checks`.

## 1.0.5

- Added a "References" section to the readme and docs.
- Added a "⬆️ Back to top" link to the end of the readme and docs, just after
 the references section.

## 1.0.4

- Added the "No standard library" category to the crate.

## 1.0.3

- Clarified that we do not depend on a specific `libm` patch version.
- Changed the `rust-version` field in `Cargo.toml` to 1.63
 since that is now the MSRV of `libm`.

## 1.0.2

- Updated `libm` dependency.

## 1.0.1

- Documentation improvements.

## 1.0.0

- Removed the `estrin` feature.
 If it was activated anywhere in the dependency tree the crate became less
 accurate for all users without them being able to do anything about it
 (as the compiler assumes that features are additive).
- Removed the `24bits` and `50bits` features. Their only use was to reduce binary
 size and speed up compile time by letting the user skip compilation of parts
 of the crate if they didn't use them. However, the crate is small and very quick
 to compile, and the unused code should be removed during dead code elimination anyway.

## 0.5.9

- Added the `LambertW` trait that lets the user call the Lambert W functions
 with postfix notation.

## 0.5.5 - 0.5.8

- Documentation improvements.

## 0.5.4

- Added `lambert_w0f` and `lambert_wm1f` functions that evaluate the 24-bit accurate
 approximation on 32-bit floats (though the approximation is expected to be
 slightly less accurate then).

## 0.5.2 and 0.5.3

- Documentation improvements.

## 0.5.1

- Added `std` and `libm` features to match the features on the optional
 dependency `fast_polynomial`.
- Made the crate `no_std` with a dependency on [`libm`](https://crates.io/crates/libm)
 by default. The crate can be made to depend on the standard library instead of
 `libm` by disabling default features and enabling the `std` feature. This can
 result in a performance gain.

## 🗑️~~0.5.0~~

Yanked because 0.5.1 adds a default feature that hides previously included
behavior.
Therefore upgrading from 0.5.0 to 0.5.1 was a breaking change if the user
had disabled default features. By yanking this version the breaking change
happens when upgrading from 0.4.4 to 0.5.1, which requires an intentional
choice by the user, and wont happen automatically with `cargo update` as before.

### Breaking changes

- Removed the last underscore in function names. E.g. `lambert_w_0` is renamed
 to `lambert_w0`. This makes them easier to type and the new names are similar
 to the names given to these functions in libraries in other languages.

## 0.4.4

- Documentation improvements.
- Updated the optional `fast_polynomial` dependency.

## 0.4.2 and 0.4.3

- Corrected a mistake in doc information.

## 0.4.1

- Added the optional `estrin` feature that computes the Lambert W function faster
 on modern hardware by using [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme)
 to evaluate the polynomials in the rational functions.
 May result in slight numerical instability, which can be reduced if the target
 CPU has fused multiply-add instructions.
- Lowered the MSRV to 1.60.0.
- It is no longer a forced `compile_error!` to disable both the `24bits` and
 `50bits` features.
- Documentation improvements.

## 0.4.0

### Breaking changes

- Made the Lambert W functions return `f64::NAN` when given inputs outside their
 domain. This is in line with how it is usually handled in the standard library.

### Other changes

- Now exports the constants `NEG_INV_E` and `OMEGA`.

## 0.3.0

### Breaking changes

- Removed the `fast` and `accurate` modules and instead export the functions directly.
- Add sp_* prefix to the 24 bit versions.

## 0.2.6

- Minor documentation improvements.

## 0.2.5

- Corrected the domain bounds in the function documentation strings.
- Other minor documentation improvements.

## 0.2.2, 0.2.3, and 0.2.4

- Documentation improvements.

## 0.2.1

- Added github repository badge to `README.md`.

## 0.2.0

### Breaking changes

- The Lambert W functions now return an `Option` instead of a `Result`
 with custom error types.

### Other changes

- The `fast` module is behind the (enabled by default) feature flag `24bits`.
- The `accurate` module is behind the (enabled by default) feature flag `50bits`.
