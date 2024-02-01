#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_println_test() {
        println!("Test");
    }
}
//#[cfg(test)]
// Esta linha indica que o código a seguir só será compilado se o projeto for configurado para compilar testes. Isso é feito usando a flag --test no comando cargo build ou cargo test.
// Isso permite manter o código de teste separado do código de produção, garantindo que o código de teste não seja incluído na versão final do programa.
// mod tests { ... }
// use super::*;
// Esta linha importa todos os nomes definidos no módulo pai (super) para o módulo tests. Isso permite que as funções e estruturas definidas no módulo pai sejam usadas dentro do módulo tests.
// #[test]
// Esta anotação é usada para marcar funções como funções de teste. As funções de teste são executadas pelo comando cargo test.