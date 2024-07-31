			Opis główny


Nasz projekt to forum dyskusyjne, skierowane do entuzjastów i profesjonalistów z rynku kryptowalut. 
Główna strona prezentuje najnowsze wpisy użytkowników, którzy dzielą się swoimi prognozami i analizami dotyczącymi różnych kryptowalut. 
Ta sekcja umożliwia uczestnikom forum wymianę opinii oraz lepsze zrozumienie bieżących i przyszłych trendów rynkowych. Członkowie 
społeczności mogą uczestniczyć w dyskusjach, śledzić interesujące ich tematy, a także dodawać nowe kryptowaluty do platformy. 
W specjalnej zakładce, użytkownicy mogą zgłaszać propozycje nowych kryptowalut oraz głosować na ich dodanie do naszej bazy danych, co zapewnia 
stałą aktualność i różnorodność dostępnych informacji. Forum Kryptowalut to idealne miejsce dla każdego, kto pragnie być na bieżąco z dynamicznie 
rozwijającym się światem cyfrowych aktywów oraz aktywnie uczestniczyć w społeczności inwestorów i entuzjastów kryptowalut.



			App.vue
App.vue to główny komponent aplikacji, który zarządza kluczowymi funkcjami i interfejsem użytkownika na stronie głównej forum kryptowalut. 
W nagłówku znajduje się logo aplikacji, które służy również jako link do strony głównej, oraz nawigacja umożliwiająca szybki dostęp do różnych sekcji 
strony, takich jak dodawanie nowej kryptowaluty czy przesyłanie postów. Pole wyszukiwania kryptowalut pozwala użytkownikom na szybkie filtrowanie 
dostępnych kryptowalut na podstawie nazwy lub skrótu, a wyniki są dynamicznie aktualizowane podczas wpisywania. Panel użytkownika wyświetla nazwę zalogowanego 
użytkownika lub przycisk logowania, a także przycisk wylogowania dla zalogowanych użytkowników.



			Home.vue

Home.vue to komponent odpowiedzialny za wyświetlanie głównej zawartości strony, w tym listy postów użytkowników związanych z kryptowalutami. 
Tło strony ma ciemny kolor, co podkreśla treści wizualne i tekstowe. W głównym obszarze znajduje się dynamiczna lista postów, która jest renderowana w 
pętli, z każdym postem zawierającym informacje takie jak nazwa użytkownika, wybrana kryptowaluta oraz przewidywania dotyczące jej wartości. Każdy post zawiera 
tekst, który może być pozytywnie lub negatywnie oceniany przez innych użytkowników za pomocą przycisków "lubię" i "nie lubię". Dodatkowo, użytkownicy mogą rozwinąć 
sekcję komentarzy dla każdego posta, gdzie mogą dodawać nowe komentarze lub oceniać istniejące.


			Login.vue 


Login.vue to komponent odpowiedzialny za logowanie użytkowników na platformie. Strona ma ciemne tło i centralnie umieszczony formularz logowania, co podkreśla jego kluczowe 
znaczenie. Nagłówek "Log in" wprowadza użytkowników do procesu logowania, a poniżej znajduje się formularz składający się z dwóch pól: dla nazwy użytkownika i hasła.
Po wypełnieniu formularza i przesłaniu go za pomocą przycisku "Log in", użytkownik jest uwierzytelniany przez backend, który sprawdza zgodność danych logowania z istniejącymi 
kontami. W przypadku błędnych danych, wyświetlany jest komunikat błędu informujący o nieprawidłowej nazwie użytkownika lub haśle.
Dla nowych użytkowników, poniżej formularza dostępna jest opcja rejestracji, która prowadzi do strony rejestracji za pomocą linku "Register". 
Komponent używa Vue Router do nawigacji i obsługuje przechowywanie danych użytkownika w store aplikacji, aby zachować stan zalogowania po udanym logowaniu.



			Rejestracja.vue

Rejestracja.vue to komponent służący do rejestracji nowych użytkowników na platformie. Strona ma ciemne tło i centralnie umieszczony formularz rejestracyjny, który zawiera 
pola dla nazwy użytkownika, adresu e-mail i hasła.Formularz wymaga również zgody na warunki użytkowania, co jest sygnalizowane przez checkbox umieszczony poniżej pól 
tekstowych. Użytkownicy mogą przesłać formularz, klikając przycisk "Register", który zmienia kolor na ciemniejszy po najechaniu, co podkreśla interaktywność.
W przypadku problemów z rejestracją, użytkownik zobaczy komunikat o błędzie pod formularzem, co ułatwia rozpoznanie i rozwiązanie problemu. Ponadto, poniżej formularza znajduje 
się link prowadzący do strony logowania dla użytkowników, którzy już mają konto. Komponent wykorzystuje router Vue do nawigacji i obsługuje komunikację z backendem w celu dodania 
nowego użytkownika. W przypadku pomyślnej rejestracji użytkownik jest przekierowywany na stronę logowania.



			UploadPost.vue


UploadPost.vue to komponent Vue, który umożliwia użytkownikom tworzenie nowych postów dotyczących kryptowalut. Strona ma ciemne tło i jest stylizowana na nowoczesny, minimalistyczny 
wygląd, z centralnie umieszczonym panelem dodawania posta. Na górze panelu znajdują się dwa pola wyboru: pierwsze umożliwia użytkownikowi wybór kryptowaluty z listy dostępnych opcji, 
które są pobierane z backendu, a drugie pozwala na określenie przewidywania kierunku rynku dla wybranej kryptowaluty. Użytkownicy mogą także wpisać tekst swojego posta w dużym polu 
tekstowym, które zajmuje większą część panelu. Przycisk "Submit Post" pozwala na przesłanie danych na serwer po wypełnieniu wymaganych pól.Komponent sprawdza, czy użytkownik jest 
zalogowany, zanim pozwoli na wysłanie posta; jeśli nie jest, wyświetla odpowiednią informację w konsoli.


			Voting.vue

Komponent Voting.vue umożliwia użytkownikom dodawanie nowych propozycji kryptowalut i głosowanie na istniejące propozycje. W sekcji dodawania propozycji użytkownicy mogą wpisać skrót 
i nazwę kryptowaluty, a następnie kliknąć przycisk "Add Proposal", aby dodać propozycję do listy. Komponent wyświetla listę propozycji z przyciskami do głosowania "Approve" (za) i 
"Reject" (przeciw), oraz aktualizuje liczby głosów w czasie rzeczywistym.

