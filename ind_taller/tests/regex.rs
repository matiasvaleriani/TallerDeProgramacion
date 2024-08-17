extern crate tp_ind;
use tp_ind::regex::Regex;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cero_o_mas() {
        let regex = Regex::new("a*b").unwrap();
        assert!(regex.test("ab").unwrap());
        assert!(regex.test("aaaaab").unwrap());
        assert!(regex.test("b").unwrap());
        assert!(!regex.test("a").unwrap());
    }

    #[test]
    fn test_una_o_mas() {
        let regex = Regex::new("a+b").unwrap();
        assert!(regex.test("ab").unwrap());
        assert!(regex.test("aaaaab").unwrap());
        assert!(!regex.test("b").unwrap());
        assert!(!regex.test("a").unwrap());
    }

    #[test]
    fn test_cero_o_una() {
        let regex = Regex::new("a?b").unwrap();
        assert!(regex.test("ab").unwrap());
        assert!(regex.test("aaaaab").unwrap());
        assert!(regex.test("b").unwrap());
        assert!(!regex.test("a").unwrap());

        let regex2 = Regex::new("ab.?d").unwrap();
        assert!(regex2.test("abcd").unwrap());
        assert!(regex2.test("abcdd").unwrap());
        assert!(regex2.test("abd").unwrap());
        assert!(regex2.test("hola abcd chau").unwrap());
        assert!(!regex2.test("abhhd").unwrap());
    }

    #[test]
    fn test_comodin() {
        let regex = Regex::new("ab.cd").unwrap();
        assert!(regex.test("ab$cd").unwrap());
        assert!(regex.test("aaaaabbcd").unwrap());
        assert!(!regex.test("abadc").unwrap());
        assert!(!regex.test("aabhhcd").unwrap());

        let regex2 = Regex::new("ab.*cd").unwrap();
        assert!(regex2.test("abgkmdfdscd").unwrap());
        assert!(regex2.test("abccccccd").unwrap());
        assert!(regex2.test("abcd").unwrap());
        assert!(!regex2.test("abfffcfff").unwrap());

        let regex3 = Regex::new("ab.*d").unwrap();
        assert!(regex3.test("absalolngopsgdehejsd").unwrap());
        assert!(regex3.test("abcdd").unwrap());
        assert!(!regex3.test("no deberia estar").unwrap());
        assert!(regex3.test("abd").unwrap());
        assert!(regex3.test("que tul abuelita dime tu").unwrap());
        assert!(regex3.test("hola abcd chau").unwrap());
    }

    #[test]
    fn test_corchete() {
        let regex = Regex::new("a[bc]d").unwrap();
        assert!(regex.test("abd").unwrap());
        assert!(regex.test("acd").unwrap());
        assert!(!regex.test("abcd").unwrap());

        let regex2 = Regex::new("[^aeiou]").unwrap();
        assert!(regex2.test("bcdfgh").unwrap());
        assert!(!regex2.test("aa").unwrap());
        assert!(!regex2.test("aeiou").unwrap());

        let regex3 = Regex::new("[[:alpha:]]+").unwrap();
        assert!(regex3.test("hola").unwrap());
        assert!(regex3.test("mundo").unwrap());
        assert!(!regex3.test("123").unwrap());

        let regex4 = Regex::new("hola[[:space:]]mundo").unwrap();
        assert!(regex4.test("hola mundo").unwrap());
        assert!(!regex4.test("holamundo").unwrap());

        let regex5 = Regex::new("[[:upper:]]ascal[[:upper:]]ase").unwrap();
        assert!(regex5.test("PascalCase").unwrap());
        assert!(!regex5.test("pascalcase").unwrap());
        assert!(regex5.test("ASDFPascalCaseASDF").unwrap());

        let regex6 = Regex::new("la [aeiou] es una vocal").unwrap();
        assert!(regex6.test("la a es una vocal").unwrap());

        let regex7 = Regex::new("la [^aeiou] no es una vocal").unwrap();
        assert!(regex7.test("la b no es una vocal").unwrap());

        let regex8 = Regex::new("hola [[:alpha:]]+").unwrap();
        assert!(regex8.test("hola mundo").unwrap());
        assert!(!regex8.test("hola 123").unwrap());

        let regex9 = Regex::new("[[:digit:]] es un numero").unwrap();
        assert!(regex9.test("1 es un numero").unwrap());
        assert!(!regex9.test("a es un numero").unwrap());

        let regex10 = Regex::new("el caracter [[:alnum:]] no es un simbolo").unwrap();
        assert!(regex10.test("el caracter 1 no es un simbolo").unwrap());
        assert!(!regex10.test("el caracter ! no es un simbolo").unwrap());
    }

    #[test]
    fn inicio() {
        let regex = Regex::new("^an").unwrap();
        assert!(regex.test("an").unwrap());
        assert!(regex.test("anana").unwrap());
        assert!(!regex.test("banana").unwrap());
    }

    #[test]
    fn fin() {
        let regex = Regex::new("an$").unwrap();
        assert!(regex.test("an").unwrap());
        assert!(regex.test("banan").unwrap());

        let regex2 = Regex::new("es el fin$").unwrap();
        assert!(regex2.test("este es el fin").unwrap());

        let regex3 = Regex::new("end$").unwrap();
        assert!(regex3.test("start middle end").unwrap());
        assert!(!regex3.test("start with whatever but end not").unwrap());
        assert!(regex3.test("end with end").unwrap());
        assert!(!regex3.test("only this line").unwrap());
    }

    #[test]
    fn llaves() {
        let regex = Regex::new("a{2}b").unwrap();
        assert!(regex.test("aab").unwrap());
        assert!(!regex.test("ab").unwrap());
        assert!(regex.test("aaab").unwrap());

        let regex_extra: Regex = Regex::new("abc{3}d").unwrap();
        assert!(!regex_extra.test("abcd").unwrap());
        assert!(regex_extra.test("abcccd").unwrap());
        assert!(regex_extra.test("hola abcccd chau").unwrap());

        let regex2 = Regex::new("a{2,}b").unwrap();
        assert!(regex2.test("aab").unwrap());
        assert!(regex2.test("aaab").unwrap());
        assert!(!regex2.test("ab").unwrap());

        let regex3 = Regex::new("ab{2,4}cd").unwrap();
        assert!(regex3.test("abbcd").unwrap());
        assert!(regex3.test("abbbcd").unwrap());
        assert!(regex3.test("abbbbcd").unwrap());
        assert!(!regex3.test("abcd").unwrap());

        let regex4 = Regex::new("a{,3}b").unwrap();
        assert!(regex4.test("b").unwrap());
        assert!(regex4.test("ab").unwrap());

        let regex5 = Regex::new("hola* noah{1,5}").unwrap();
        assert!(regex5.test("holaaaaaaaaaa noahhhhhh").unwrap());
        assert!(regex5.test("hol noah").unwrap());
        assert!(regex5.test("holaa noahhh").unwrap());
        assert!(!regex5.test("pepito").unwrap());
        assert!(!regex5.test("me presento noah soy hola").unwrap());
        assert!(regex5.test("soy hola noah chau").unwrap());
    }

    #[test]
    fn mas_de_una_llave() {
        let regex = Regex::new("abc{2,5}d abc{0,}d").unwrap();
        assert!(!regex.test("abcd abcd").unwrap());
        assert!(regex.test("abd abcccd abd").unwrap());
        assert!(!regex.test("abcccccccd abcd").unwrap());
        assert!(regex.test("en medio abccd abd fin").unwrap());
    }

    #[test]
    fn alternancia() {
        let regex = Regex::new("a|b").unwrap();
        assert!(regex.test("a").unwrap());
        assert!(regex.test("b").unwrap());
        assert!(!regex.test("c").unwrap());
        assert!(regex.test("ab").unwrap());

        let regex2 = Regex::new("abc|de+f").unwrap();
        assert!(regex2.test("abc").unwrap());
        assert!(regex2.test("def").unwrap());
        assert!(regex2.test("deef").unwrap());
        assert!(regex2.test("deeeef").unwrap());

        let regex3 = Regex::new("a{2,}b|c{3,}d").unwrap();
        assert!(regex3.test("aab").unwrap());
        assert!(regex3.test("aaab").unwrap());

        let regex4 = Regex::new("abc\\?def|123\\*456|789\\+10").unwrap();
        assert!(regex4.test("abc?def").unwrap());
        assert!(regex4.test("123*456").unwrap());
        assert!(regex4.test("789+10").unwrap());
        assert!(!regex4.test("hola?").unwrap());
        assert!(!regex4.test("esta no tiene que estar").unwrap());
    }
}
