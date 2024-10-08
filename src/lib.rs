#![cfg_attr(feature = "_nightly-toolchain", feature(test))]
#![cfg_attr(target_arch = "wasm32", recursion_limit = "512")]

pub mod geometry;
pub mod image;
pub mod rdp;
pub mod triangulation;

#[cfg(test)]
mod benches {
    extern crate test;

    use super::{image::ImagePolygon, rdp::rdp, triangulation::triangulate};
    use crate::geometry::Triangle;
    use test::Bencher;

    #[bench]
    #[cfg_attr(target_arch = "wasm32", ignore)]
    fn image_interpretation(b: &mut Bencher) {
        if std::env::var("CAZAN_IMAGE_PATH").is_err() {
            return;
        }

        b.iter(|| {
            let image = image::open(std::env::var("CAZAN_IMAGE_PATH").unwrap())
                .expect("Error opening image");
            let (polygon, holes) = ImagePolygon::new(image).to_polygon();
            let rdp_polygon = rdp(&polygon, 1.0);
            let rdp_holes = &holes.iter().map(|hole| rdp(hole, 1.0)).collect();
            triangulate(&rdp_polygon, Some(rdp_holes)).expect("Error triangulating");
        });
    }

    #[bench]
    fn minkowski_difference(b: &mut Bencher) {
        let t1: Vec<Triangle>;
        let t2: Vec<Triangle>;

        #[cfg(not(target_arch = "wasm32"))]
        {
            if std::env::var("CAZAN_IMAGE_PATH").is_err() {
                return;
            }

            let image = image::open(std::env::var("CAZAN_IMAGE_PATH").unwrap())
                .expect("Error opening image");
            let (polygon, holes) = ImagePolygon::new(image).to_polygon();
            let rdp_polygon = rdp(&polygon, 1.0);
            let rdp_holes = &holes.iter().map(|hole| rdp(hole, 1.0)).collect();
            t1 = triangulate(&rdp_polygon, Some(rdp_holes)).expect("Error triangulating");
            t2 = t1.clone();
        }

        #[cfg(target_arch = "wasm32")]
        {
            use serde_json::json;
            use std::io::Read;
            // Show current dir

            // This is the pikachu.png triangulation
            let contents = json!(
                [[{"x":82,"y":85},{"x":82,"y":84},{"x":102,"y":85}],[{"x":102,"y":85},{"x":129,"y":92},{"x":188,"y":117}],[{"x":188,"y":117},{"x":218,"y":132},{"x":246,"y":150}],[{"x":246,"y":150},{"x":260,"y":161},{"x":303,"y":200}],[{"x":336,"y":225},{"x":346,"y":219},{"x":369,"y":210}],[{"x":374,"y":210},{"x":382,"y":207},{"x":388,"y":207}],[{"x":388,"y":207},{"x":403,"y":203},{"x":428,"y":202}],[{"x":428,"y":202},{"x":459,"y":203},{"x":475,"y":207}],[{"x":475,"y":207},{"x":481,"y":207},{"x":499,"y":212}],[{"x":501,"y":214},{"x":505,"y":214},{"x":518,"y":221}],[{"x":518,"y":221},{"x":521,"y":221},{"x":526,"y":225}],[{"x":531,"y":225},{"x":537,"y":221},{"x":591,"y":203}],[{"x":591,"y":203},{"x":596,"y":200},{"x":600,"y":200}],[{"x":600,"y":200},{"x":605,"y":197},{"x":644,"y":185}],[{"x":650,"y":185},{"x":660,"y":182},{"x":669,"y":182}],[{"x":669,"y":182},{"x":688,"y":178},{"x":700,"y":178}],[{"x":700,"y":178},{"x":714,"y":175},{"x":731,"y":175}],[{"x":731,"y":175},{"x":746,"y":173},{"x":783,"y":173}],[{"x":783,"y":173},{"x":802,"y":175},{"x":810,"y":178}],[{"x":810,"y":178},{"x":809,"y":184},{"x":784,"y":207}],[{"x":784,"y":207},{"x":759,"y":224},{"x":728,"y":241}],[{"x":728,"y":241},{"x":684,"y":256},{"x":663,"y":260}],[{"x":663,"y":260},{"x":651,"y":264},{"x":645,"y":264}],[{"x":645,"y":264},{"x":616,"y":271},{"x":596,"y":272}],[{"x":579,"y":275},{"x":589,"y":300},{"x":589,"y":306}],[{"x":589,"y":306},{"x":592,"y":315},{"x":592,"y":325}],[{"x":592,"y":325},{"x":596,"y":344},{"x":596,"y":352}],[{"x":596,"y":352},{"x":599,"y":361},{"x":599,"y":367}],[{"x":599,"y":367},{"x":603,"y":384},{"x":604,"y":410}],[{"x":604,"y":410},{"x":606,"y":418},{"x":606,"y":427}],[{"x":606,"y":427},{"x":602,"y":446},{"x":585,"y":481}],[{"x":585,"y":498},{"x":596,"y":556},{"x":596,"y":580}],[{"x":596,"y":580},{"x":599,"y":602},{"x":599,"y":618}],[{"x":599,"y":618},{"x":603,"y":633},{"x":603,"y":639}],[{"x":624,"y":641},{"x":607,"y":585},{"x":600,"y":549}],[{"x":600,"y":549},{"x":609,"y":546},{"x":616,"y":546}],[{"x":616,"y":546},{"x":633,"y":542},{"x":684,"y":537}],[{"x":684,"y":537},{"x":635,"y":405},{"x":635,"y":401}],[{"x":635,"y":401},{"x":632,"y":395},{"x":625,"y":367}],[{"x":625,"y":367},{"x":627,"y":363},{"x":647,"y":357}],[{"x":652,"y":357},{"x":695,"y":346},{"x":701,"y":346}],[{"x":701,"y":346},{"x":713,"y":342},{"x":719,"y":342}],[{"x":719,"y":342},{"x":748,"y":335},{"x":755,"y":335}],[{"x":755,"y":335},{"x":764,"y":332},{"x":771,"y":332}],[{"x":771,"y":332},{"x":785,"y":328},{"x":809,"y":325}],[{"x":809,"y":325},{"x":824,"y":321},{"x":832,"y":321}],[{"x":832,"y":321},{"x":848,"y":317},{"x":945,"y":303}],[{"x":945,"y":303},{"x":974,"y":302},{"x":975,"y":304}],[{"x":975,"y":304},{"x":971,"y":321},{"x":953,"y":363}],[{"x":953,"y":363},{"x":953,"y":366},{"x":949,"y":372}],[{"x":949,"y":372},{"x":948,"y":377},{"x":935,"y":403}],[{"x":935,"y":403},{"x":935,"y":406},{"x":924,"y":426}],[{"x":924,"y":426},{"x":921,"y":435},{"x":895,"y":485}],[{"x":895,"y":485},{"x":882,"y":485},{"x":862,"y":482}],[{"x":862,"y":482},{"x":845,"y":482},{"x":751,"y":472}],[{"x":745,"y":473},{"x":764,"y":536},{"x":772,"y":569}],[{"x":772,"y":569},{"x":774,"y":572},{"x":774,"y":577}],[{"x":774,"y":577},{"x":782,"y":605},{"x":783,"y":616}],[{"x":783,"y":616},{"x":741,"y":614},{"x":662,"y":603}],[{"x":661,"y":606},{"x":671,"y":638},{"x":674,"y":655}],[{"x":674,"y":655},{"x":678,"y":666},{"x":681,"y":685}],[{"x":681,"y":685},{"x":656,"y":685},{"x":621,"y":680}],[{"x":617,"y":698},{"x":620,"y":705},{"x":628,"y":751}],[{"x":628,"y":751},{"x":628,"y":792},{"x":624,"y":814}],[{"x":624,"y":814},{"x":617,"y":833},{"x":602,"y":859}],[{"x":602,"y":859},{"x":592,"y":870},{"x":588,"y":872}],[{"x":616,"y":879},{"x":627,"y":880},{"x":652,"y":887}],[{"x":671,"y":897},{"x":676,"y":898},{"x":676,"y":908}],[{"x":676,"y":908},{"x":660,"y":916},{"x":646,"y":919}],[{"x":646,"y":919},{"x":643,"y":921},{"x":632,"y":922}],[{"x":632,"y":922},{"x":625,"y":925},{"x":611,"y":926}],[{"x":611,"y":926},{"x":598,"y":929},{"x":567,"y":931}],[{"x":567,"y":931},{"x":560,"y":933},{"x":540,"y":933}],[{"x":540,"y":933},{"x":495,"y":937},{"x":366,"y":936}],[{"x":346,"y":933},{"x":325,"y":933},{"x":318,"y":931}],[{"x":318,"y":931},{"x":291,"y":929},{"x":286,"y":927}],[{"x":286,"y":927},{"x":271,"y":926},{"x":247,"y":921}],[{"x":247,"y":921},{"x":220,"y":912},{"x":214,"y":905}],[{"x":214,"y":905},{"x":214,"y":902},{"x":222,"y":895}],[{"x":222,"y":895},{"x":237,"y":887},{"x":256,"y":881}],[{"x":282,"y":876},{"x":277,"y":874},{"x":268,"y":867}],[{"x":268,"y":867},{"x":253,"y":844},{"x":239,"y":807}],[{"x":239,"y":807},{"x":235,"y":785},{"x":235,"y":772}],[{"x":239,"y":747},{"x":239,"y":738},{"x":246,"y":710}],[{"x":246,"y":710},{"x":246,"y":700},{"x":250,"y":688}],[{"x":250,"y":688},{"x":250,"y":681},{"x":264,"y":618}],[{"x":264,"y":618},{"x":264,"y":597},{"x":267,"y":574}],[{"x":267,"y":574},{"x":267,"y":550},{"x":277,"y":499}],[{"x":276,"y":476},{"x":264,"y":455},{"x":257,"y":432}],[{"x":257,"y":432},{"x":257,"y":411},{"x":260,"y":390}],[{"x":260,"y":390},{"x":260,"y":379},{"x":264,"y":363}],[{"x":264,"y":363},{"x":264,"y":357},{"x":267,"y":349}],[{"x":267,"y":349},{"x":267,"y":340},{"x":270,"y":327}],[{"x":270,"y":327},{"x":272,"y":307},{"x":278,"y":285}],[{"x":274,"y":264},{"x":264,"y":260},{"x":235,"y":242}],[{"x":235,"y":242},{"x":233,"y":242},{"x":202,"y":221}],[{"x":202,"y":221},{"x":150,"y":177},{"x":103,"y":127}],[{"x":103,"y":127},{"x":85,"y":98},{"x":82,"y":92}],[{"x":82,"y":92},{"x":82,"y":85},{"x":102,"y":85}],[{"x":102,"y":85},{"x":188,"y":117},{"x":246,"y":150}],[{"x":246,"y":150},{"x":303,"y":200},{"x":316,"y":216}],[{"x":336,"y":225},{"x":369,"y":210},{"x":374,"y":210}],[{"x":374,"y":210},{"x":388,"y":207},{"x":428,"y":202}],[{"x":428,"y":202},{"x":475,"y":207},{"x":499,"y":212}],[{"x":501,"y":214},{"x":518,"y":221},{"x":526,"y":225}],[{"x":531,"y":225},{"x":591,"y":203},{"x":600,"y":200}],[{"x":600,"y":200},{"x":644,"y":185},{"x":650,"y":185}],[{"x":650,"y":185},{"x":669,"y":182},{"x":700,"y":178}],[{"x":700,"y":178},{"x":731,"y":175},{"x":783,"y":173}],[{"x":783,"y":173},{"x":810,"y":178},{"x":784,"y":207}],[{"x":784,"y":207},{"x":728,"y":241},{"x":663,"y":260}],[{"x":663,"y":260},{"x":645,"y":264},{"x":596,"y":272}],[{"x":579,"y":275},{"x":589,"y":306},{"x":592,"y":325}],[{"x":596,"y":352},{"x":599,"y":367},{"x":604,"y":410}],[{"x":604,"y":410},{"x":606,"y":427},{"x":585,"y":481}],[{"x":585,"y":498},{"x":596,"y":580},{"x":599,"y":618}],[{"x":624,"y":641},{"x":600,"y":549},{"x":616,"y":546}],[{"x":684,"y":537},{"x":635,"y":401},{"x":625,"y":367}],[{"x":625,"y":367},{"x":647,"y":357},{"x":652,"y":357}],[{"x":652,"y":357},{"x":701,"y":346},{"x":719,"y":342}],[{"x":719,"y":342},{"x":755,"y":335},{"x":771,"y":332}],[{"x":771,"y":332},{"x":809,"y":325},{"x":832,"y":321}],[{"x":832,"y":321},{"x":945,"y":303},{"x":975,"y":304}],[{"x":975,"y":304},{"x":953,"y":363},{"x":949,"y":372}],[{"x":949,"y":372},{"x":935,"y":403},{"x":924,"y":426}],[{"x":924,"y":426},{"x":895,"y":485},{"x":862,"y":482}],[{"x":745,"y":473},{"x":772,"y":569},{"x":774,"y":577}],[{"x":774,"y":577},{"x":783,"y":616},{"x":662,"y":603}],[{"x":661,"y":606},{"x":674,"y":655},{"x":681,"y":685}],[{"x":617,"y":698},{"x":628,"y":751},{"x":624,"y":814}],[{"x":624,"y":814},{"x":602,"y":859},{"x":588,"y":872}],[{"x":616,"y":879},{"x":652,"y":887},{"x":671,"y":897}],[{"x":671,"y":897},{"x":676,"y":908},{"x":646,"y":919}],[{"x":646,"y":919},{"x":632,"y":922},{"x":611,"y":926}],[{"x":611,"y":926},{"x":567,"y":931},{"x":540,"y":933}],[{"x":540,"y":933},{"x":366,"y":936},{"x":346,"y":933}],[{"x":346,"y":933},{"x":318,"y":931},{"x":286,"y":927}],[{"x":286,"y":927},{"x":247,"y":921},{"x":214,"y":905}],[{"x":214,"y":905},{"x":222,"y":895},{"x":256,"y":881}],[{"x":282,"y":876},{"x":268,"y":867},{"x":239,"y":807}],[{"x":239,"y":807},{"x":235,"y":772},{"x":239,"y":747}],[{"x":246,"y":710},{"x":250,"y":688},{"x":264,"y":618}],[{"x":264,"y":618},{"x":267,"y":574},{"x":277,"y":499}],[{"x":276,"y":476},{"x":257,"y":432},{"x":260,"y":390}],[{"x":260,"y":390},{"x":264,"y":363},{"x":267,"y":349}],[{"x":267,"y":349},{"x":270,"y":327},{"x":278,"y":285}],[{"x":274,"y":264},{"x":235,"y":242},{"x":202,"y":221}],[{"x":202,"y":221},{"x":103,"y":127},{"x":82,"y":92}],[{"x":82,"y":92},{"x":102,"y":85},{"x":246,"y":150}],[{"x":336,"y":225},{"x":374,"y":210},{"x":428,"y":202}],[{"x":428,"y":202},{"x":499,"y":212},{"x":501,"y":214}],[{"x":531,"y":225},{"x":600,"y":200},{"x":650,"y":185}],[{"x":650,"y":185},{"x":700,"y":178},{"x":783,"y":173}],[{"x":783,"y":173},{"x":784,"y":207},{"x":663,"y":260}],[{"x":663,"y":260},{"x":596,"y":272},{"x":579,"y":275}],[{"x":579,"y":275},{"x":592,"y":325},{"x":596,"y":352}],[{"x":596,"y":352},{"x":604,"y":410},{"x":585,"y":481}],[{"x":624,"y":641},{"x":616,"y":546},{"x":684,"y":537}],[{"x":684,"y":537},{"x":625,"y":367},{"x":652,"y":357}],[{"x":652,"y":357},{"x":719,"y":342},{"x":771,"y":332}],[{"x":771,"y":332},{"x":832,"y":321},{"x":975,"y":304}],[{"x":975,"y":304},{"x":949,"y":372},{"x":924,"y":426}],[{"x":924,"y":426},{"x":862,"y":482},{"x":751,"y":472}],[{"x":745,"y":473},{"x":774,"y":577},{"x":662,"y":603}],[{"x":661,"y":606},{"x":681,"y":685},{"x":621,"y":680}],[{"x":614,"y":681},{"x":617,"y":698},{"x":624,"y":814}],[{"x":588,"y":872},{"x":616,"y":879},{"x":671,"y":897}],[{"x":671,"y":897},{"x":646,"y":919},{"x":611,"y":926}],[{"x":611,"y":926},{"x":540,"y":933},{"x":346,"y":933}],[{"x":346,"y":933},{"x":286,"y":927},{"x":214,"y":905}],[{"x":214,"y":905},{"x":256,"y":881},{"x":268,"y":880}],[{"x":282,"y":876},{"x":239,"y":807},{"x":239,"y":747}],[{"x":239,"y":747},{"x":246,"y":710},{"x":264,"y":618}],[{"x":276,"y":476},{"x":260,"y":390},{"x":267,"y":349}],[{"x":267,"y":349},{"x":278,"y":285},{"x":285,"y":272}],[{"x":274,"y":264},{"x":202,"y":221},{"x":82,"y":92}],[{"x":82,"y":92},{"x":246,"y":150},{"x":316,"y":216}],[{"x":336,"y":225},{"x":428,"y":202},{"x":501,"y":214}],[{"x":531,"y":225},{"x":650,"y":185},{"x":783,"y":173}],[{"x":783,"y":173},{"x":663,"y":260},{"x":579,"y":275}],[{"x":579,"y":275},{"x":596,"y":352},{"x":585,"y":481}],[{"x":623,"y":643},{"x":624,"y":641},{"x":684,"y":537}],[{"x":684,"y":537},{"x":652,"y":357},{"x":771,"y":332}],[{"x":771,"y":332},{"x":975,"y":304},{"x":924,"y":426}],[{"x":924,"y":426},{"x":751,"y":472},{"x":745,"y":473}],[{"x":662,"y":603},{"x":661,"y":606},{"x":621,"y":680}],[{"x":614,"y":681},{"x":624,"y":814},{"x":588,"y":872}],[{"x":588,"y":872},{"x":671,"y":897},{"x":611,"y":926}],[{"x":611,"y":926},{"x":346,"y":933},{"x":214,"y":905}],[{"x":214,"y":905},{"x":268,"y":880},{"x":282,"y":876}],[{"x":282,"y":876},{"x":239,"y":747},{"x":264,"y":618}],[{"x":276,"y":476},{"x":267,"y":349},{"x":285,"y":272}],[{"x":285,"y":272},{"x":274,"y":264},{"x":82,"y":92}],[{"x":82,"y":92},{"x":316,"y":216},{"x":328,"y":227}],[{"x":328,"y":227},{"x":336,"y":225},{"x":501,"y":214}],[{"x":531,"y":225},{"x":783,"y":173},{"x":579,"y":275}],[{"x":579,"y":275},{"x":585,"y":481},{"x":585,"y":498}],[{"x":771,"y":332},{"x":924,"y":426},{"x":745,"y":473}],[{"x":662,"y":603},{"x":621,"y":680},{"x":614,"y":681}],[{"x":588,"y":872},{"x":611,"y":926},{"x":214,"y":905}],[{"x":282,"y":876},{"x":264,"y":618},{"x":277,"y":499}],[{"x":277,"y":499},{"x":276,"y":476},{"x":285,"y":272}],[{"x":285,"y":272},{"x":82,"y":92},{"x":328,"y":227}],[{"x":328,"y":227},{"x":501,"y":214},{"x":526,"y":225}],[{"x":526,"y":225},{"x":531,"y":225},{"x":579,"y":275}],[{"x":684,"y":537},{"x":771,"y":332},{"x":745,"y":473}],[{"x":588,"y":872},{"x":214,"y":905},{"x":282,"y":876}],[{"x":282,"y":876},{"x":277,"y":499},{"x":285,"y":272}],[{"x":285,"y":272},{"x":328,"y":227},{"x":526,"y":225}],[{"x":526,"y":225},{"x":579,"y":275},{"x":585,"y":498}],[{"x":623,"y":643},{"x":684,"y":537},{"x":745,"y":473}],[{"x":614,"y":681},{"x":588,"y":872},{"x":282,"y":876}],[{"x":282,"y":876},{"x":285,"y":272},{"x":526,"y":225}],[{"x":526,"y":225},{"x":585,"y":498},{"x":599,"y":618}],[{"x":623,"y":643},{"x":745,"y":473},{"x":662,"y":603}],[{"x":282,"y":876},{"x":526,"y":225},{"x":599,"y":618}],[{"x":623,"y":643},{"x":662,"y":603},{"x":614,"y":681}],[{"x":282,"y":876},{"x":599,"y":618},{"x":603,"y":639}],[{"x":606,"y":644},{"x":623,"y":643},{"x":614,"y":681}],[{"x":614,"y":681},{"x":282,"y":876},{"x":603,"y":639}],[{"x":603,"y":639},{"x":606,"y":644},{"x":614,"y":681}]]
            );

            t1 = serde_json::from_value(contents).unwrap();
            t2 = t1.clone();
        }

        assert_eq!(t1.len(), t2.len());

        b.iter(|| {
            for triangle in t1.iter() {
                for triangle2 in t2.iter() {
                    for point in triangle.into_iter() {
                        for point2 in triangle2.into_iter() {
                            let x = point.x as i32 - point2.x as i32;
                            let y = point.y as i32 - point2.y as i32;
                            let _ = (x, y);
                        }
                    }
                }
            }
        });
    }
}
