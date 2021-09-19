
use yew::prelude::*;
use crate::components::resources::category::Category;

pub struct Resources {}

impl Component for Resources {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Resources {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section class="resources-section">
            <div class="resources-title-container">
                <span class="resources-title">{"GUÍA DE RECURSOS"}</span>
                <p>{"Aqui reunimos algunos recursos didácticos para aprender Rust."}</p>
            </div>
            <div class="resources-container-list">
                <div class="resources-left-side">
                    <Category title="Principiante">
                        <p>{"En general recomendamos comenzar por el libro oficial, porque es el principal recurso oficial que se mantiene actualizado a la par del lenguaje."}</p>
                        <a href="https://doc.rust-lang.org/book/">{"The Rust Programming Language"}</a>
                        <p>{"El punto de inicio de toda la documentacion oficial se encuentra "}
                        <a href="https://www.rust-lang.org/learn">{"aqui"}</a>
                        {". De particular interes inmediato a principiantes es la documentación de la"} 
                        <a href="https://doc.rust-lang.org/std/index.html">{"librería estandar"}</a>
                        {" y la documentación de "}
                        <a href="https://doc.rust-lang.org/cargo/index.html">{"Cargo"}</a>
                        {"."}</p>
                        <p>{"Para iniciar una práctica guiada se recomienda el curso de "}
                        <a href="https://github.com/rust-lang/rustlings/">{"Rustlings"}</a>{" y para ver ejemplos de código aplicado "}
                        <a href="https://doc.rust-lang.org/stable/rust-by-example/">{"Rust by Example"}</a>
                        {"."}</p>
                        <p>{"Otros recursos interesantes:"}</p>
                        <ul>
                            <li><a href="http://intorust.com/">{"Into Rust"}</a></li>
                            <li><a href="https://learning-rust.github.io/">{"Learning Rust"}</a></li>
                        </ul>
                    </Category>
                    <Category title="Intermedio">
                        <p>{"No tenemos pero puedes contribuir si deseas :D"}</p>
                    </Category>
                    <Category title="Avanzado">
                        <a href="https://rust-lang-nursery.github.io/rust-cookbook/">{"The Rust Cookbook"}</a>
                        <p>{"Un recurso avanzado es "}<a href="https://rust-unofficial.github.io/too-many-lists/">{"Learn Rust With Entirely Too Many Linked Lists"}</a></p>
                    </Category>
                    <br/>
                    <p>{"No todas las personas tienen el mismo estilo de aprendizaje, y algunas prefieren el formato audiovisual.
El equipo de desarrollo de Rust tiene un canal oficial en "}
                    <a href="https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA">{"Youtube"}</a>
                    {" y "}
                    <a href="https://www.youtube.com/playlist?list=PLLqEtX6ql2EyPAZ1M2_C0GgVd4A-_L4_5">{"esta serie de videos"}</a>
                    {" provee una introducción al lenguaje en inglés."}</p>
                </div>
                <div class="resources-right-side">
                    <img src="images/rust_arg_web.svg"  />
                </div>
            </div>
          </section>
        }
    }
}
