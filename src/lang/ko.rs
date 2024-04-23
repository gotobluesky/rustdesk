lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "상태"),
        ("Your Desktop", "내 데스크탑"),
        ("desk_tip", "아래의 ID와 비밀번호로 연결할수 있습니다"),
        ("Password", "비밀번호"),
        ("Ready", "준비"),
        ("Established", "연결됨"),
        ("connecting_status", "RustDesk 네트워크로 연결중입니다..."),
        ("Enable service", "서비스 활성화"),
        ("Start service", "서비스 시작"),
        ("Service is running", "서비스가 실행되었습니다"),
        ("Service is not running", "서비스가 실행되지 않았습니다"),
        ("not_ready_status", "준비되지 않았습니다. 연결을 확인해주세요."),
        ("Control Remote Desktop", "원격 데스크탑 제어"),
        ("Transfer file", "파일 전송"),
        ("Connect", "연결하기"),
        ("Recent sessions", "최근 세션"),
        ("Address book", "세션 주소록"),
        ("Confirmation", "확인"),
        ("TCP tunneling", "TCP 터널링"),
        ("Remove", "삭제"),
        ("Refresh random password", "랜덤 비밀번호 변경"),
        ("Set your own password", "개인 비밀번호 설정"),
        ("Enable keyboard/mouse", "키보드/마우스 활성화"),
        ("Enable clipboard", "클립보드 활성화"),
        ("Enable file transfer", "파일 전송 활성화"),
        ("Enable TCP tunneling", "TCP 터널링 활성화"),
        ("IP Whitelisting", "IP 화이트리스트"),
        ("ID/Relay Server", "ID/릴레이 서버"),
        ("Import server config", "서버 설정 가져오기"),
        ("Export Server Config", "서버 설정 내보내기"),
        ("Import server configuration successfully", "서버 설정 가져오기 성공"),
        ("Export server configuration successfully", "서버 설정 내보내기 성공"),
        ("Invalid server configuration", "잘못된 서버 설정"),
        ("Clipboard is empty", "클립보드가 비어있습니다"),
        ("Stop service", "서비스 중지"),
        ("Change ID", "ID 변경"),
        ("Your new ID", "당신의 새로운 ID"),
        ("length %min% to %max%", "길이 %min% ~ %max%"),
        ("starts with a letter", "문자로 시작해야 합니다"),
        ("allowed characters", "허용되는 문자"),
        ("id_change_tip", "a-z, A-Z, 0-9, _(언더바)만 입력 가능합니다. 첫 문자는 a-z 혹은 A-Z로 시작해야 합니다. 길이는 6~16글자가 요구됩니다."),
        ("Website", "웹사이트"),
        ("About", "정보"),
        ("Slogan_tip", "이 혼란스러운 세상에서 마음을 담아 만들었습니다!"),
        ("Privacy Statement", "개인 정보 보호 정책"),
        ("Mute", "음소거"),
        ("Build Date", "빌드 날짜"),
        ("Version", "버전"),
        ("Home", "홈"),
        ("Audio Input", "오디오 입력"),
        ("Enhancements", "향상된 기능"),
        ("Hardware Codec", "하드웨어 코덱"),
        ("Adaptive bitrate", "가변 비트레이트"),
        ("ID Server", "ID 서버"),
        ("Relay Server", "릴레이 서버"),
        ("API Server", "API 서버"),
        ("invalid_http", "http:// 또는 https:// 로 시작해야합니다"),
        ("Invalid IP", "유효하지 않은 IP"),
        ("Invalid format", "유효하지 않은 형식"),
        ("server_not_support", "아직 서버에서 지원되지 않습니다"),
        ("Not available", "불가능"),
        ("Too frequent", "수정이 너무 자주 발생합니다. 나중에 재시도해 주세요."),
        ("Cancel", "취소"),
        ("Skip", "넘기기"),
        ("Close", "닫기"),
        ("Retry", "재시도"),
        ("OK", "확인"),
        ("Password Required", "비밀번호 입력"),
        ("Please enter your password", "비밀번호를 입력해주세요"),
        ("Remember password", "비밀번호 기억하기"),
        ("Wrong Password", "비밀번호가 다릅니다"),
        ("Do you want to enter again?", "다시 연결하시겠습니까?"),
        ("Connection Error", "연결 오류"),
        ("Error", "오류"),
        ("Reset by the peer", "다른 접속자에 의해 초기화됨"),
        ("Connecting...", "연결중..."),
        ("Connection in progress. Please wait.", "연결중입니다. 잠시만 기다려주세요"),
        ("Please try 1 minute later", "1분 후에 재시도하세요"),
        ("Login Error", "로그인 오류"),
        ("Successful", "성공"),
        ("Connected, waiting for image...", "연결됨. 화면 전송 대기 중..."),
        ("Name", "이름"),
        ("Type", "유형"),
        ("Modified", "수정됨"),
        ("Size", "크기"),
        ("Show Hidden Files", "숨겨진 파일 표시"),
        ("Receive", "받기"),
        ("Send", "보내기"),
        ("Refresh File", "파일 새로고침"),
        ("Local", "로컬"),
        ("Remote", "원격"),
        ("Remote Computer", "원격 컴퓨터"),
        ("Local Computer", "로컬 컴퓨터"),
        ("Confirm Delete", "삭제 확인"),
        ("Delete", "삭제"),
        ("Properties", "속성"),
        ("Multi Select", "다중 선택"),
        ("Select All", "전체 선택"),
        ("Unselect All", "전체 선택 해제"),
        ("Empty Directory", "폴더가 비어있습니다"),
        ("Not an empty directory", "폴더가 비어있지 않습니다"),
        ("Are you sure you want to delete this file?", "이 파일을 삭제하시겠습니까?"),
        ("Are you sure you want to delete this empty directory?", "이 빈 폴더를 삭제하시겠습니까?"),
        ("Are you sure you want to delete the file of this directory?", "이 폴더의 파일을 삭제하시겠습니까?"),
        ("Do this for all conflicts", "모든 충돌에 대해 이 작업을 수행합니다"),
        ("This is irreversible!", "이 작업은 되돌릴 수 없습니다.!"),
        ("Deleting", "삭제중"),
        ("files", "파일"),
        ("Waiting", "대기중"),
        ("Finished", "완료됨"),
        ("Speed", "속도"),
        ("Custom Image Quality", "화질 설정"),
        ("Privacy mode", "개인정보 보호 모드"),
        ("Block user input", "사용자 입력 차단"),
        ("Unblock user input", "사용자 입력 차단 해제"),
        ("Adjust Window", "화면 조정"),
        ("Original", "원본"),
        ("Shrink", "축소"),
        ("Stretch", "확대"),
        ("Scrollbar", "스크롤바"),
        ("ScrollAuto", "자동스크롤"),
        ("Good image quality", "이미지 품질 최적화"),
        ("Balanced", "균형"),
        ("Optimize reaction time", "반응 시간 최적화"),
        ("Custom", "사용자 정의"),
        ("Show remote cursor", "원격 커서 보이기"),
        ("Show quality monitor", "품질 모니터 보기"),
        ("Disable clipboard", "클립보드 비활성화"),
        ("Lock after session end", "세션 종료 후 화면 잠금"),
        ("Insert", "입력"),
        ("Insert Lock", "원격 입력 잠금"),
        ("Refresh", "새로고침"),
        ("ID does not exist", "ID가 존재하지 않습니다"),
        ("Failed to connect to rendezvous server", "등록 서버에 연결하지 못했습니다."),
        ("Please try later", "재시도해주세요"),
        ("Remote desktop is offline", "원격 데스크탑이 연결되어 있지 않습니다"),
        ("Key mismatch", "키가 일치하지 않습니다."),
        ("Timeout", "시간 초과"),
        ("Failed to connect to relay server", "릴레이 서버 연결에 실패하였습니다"),
        ("Failed to connect via rendezvous server", "등록 서버를 통한 연결에 실패하였습니다"),
        ("Failed to connect via relay server", "릴레이 서버를 통한 연결에 실패하였습니다"),
        ("Failed to make direct connection to remote desktop", "원격 데스크탑으로의 직접 연결 생성에 실패하였습니다"),
        ("Set Password", "비밀번호 설정"),
        ("OS Password", "OS 비밀번호"),
        ("install_tip", "UAC로 인해, RustDesk가 원격지일 때 일부 기능이 동작하지 않을 수 있습니다. UAC 문제를 방지하려면, 아래 버튼을 클릭하여 RustDesk를 시스템에 설치해주세요."),
        ("Click to upgrade", "업그레이드"),
        ("Click to download", "다운로드"),
        ("Click to update", "업데이트"),
        ("Configure", "구성"),
        ("config_acc", "내 데스크탑을 원격제어하기 전에, RustDesk에게 \"Accessibility (접근성)\" 권한을 부여해야 합니다."),
        ("config_screen", "내 데스크탑을 원격제어하기 전에, RustDesk에게 \"Screen Recording (화면 녹화)\" 권한을 부여해야 합니다."),
        ("Installing ...", "설치중 ..."),
        ("Install", "설치하기"),
        ("Installation", "설치"),
        ("Installation Path", "설치 경로"),
        ("Create start menu shortcuts", "시작 메뉴에 바로가기 생성"),
        ("Create desktop icon", "데스크탑 아이콘 생성"),
        ("agreement_tip", "설치를 시작하기 전에, 라이선스 약관에 동의를 해야합니다."),
        ("Accept and Install", "동의 및 설치"),
        ("End-user license agreement", "최종 사용자 라이선스 약관 동의"),
        ("Generating ...", "생성중 ..."),
        ("Your installation is lower version.", "설치한 버전이 현재 실행 중인 버전보다 낮습니다."),
        ("not_close_tcp_tip", "연결중에는 이 창을 끄지 마세요"),
        ("Listening ...", "연결 대기중 ..."),
        ("Remote Host", "원격 호스트"),
        ("Remote Port", "원격 포트"),
        ("Action", "액션"),
        ("Add", "추가"),
        ("Local Port", "로컬 포트"),
        ("Local Address", "현재 주소"),
        ("Change Local Port", "로컬 포트 변경"),
        ("setup_server_tip", "자체 서버를 구축하면 더 빠른 속도로 사용할수 있습니다"),
        ("Too short, at least 6 characters.", "너무 짧습니다, 최소 6글자 이상 입력해주세요."),
        ("The confirmation is not identical.", "두 입력이 일치하지 않습니다."),
        ("Permissions", "권한"),
        ("Accept", "수락"),
        ("Dismiss", "거부"),
        ("Disconnect", "연결 종료"),
        ("Enable file copy and paste", "파일 복사 및 붙여넣기 허용"),
        ("Connected", "연결됨"),
        ("Direct and encrypted connection", "암호화된 다이렉트 연결"),
        ("Relayed and encrypted connection", "암호화된 릴레이 연결"),
        ("Direct and unencrypted connection", "암호화되지 않은 다이렉트 연결"),
        ("Relayed and unencrypted connection", "암호화되지 않은 릴레이 연결"),
        ("Enter Remote ID", "원격 ID를 입력하세요"),
        ("Enter your password", "비밀번호를 입력하세요"),
        ("Logging in...", "로그인 중..."),
        ("Enable RDP session sharing", "RDP 세션 공유 활성화"),
        ("Auto Login", "자동 로그인"),
        ("Enable direct IP access", "다이렉트 IP 연결 활성화"),
        ("Rename", "이름 변경"),
        ("Space", "공간"),
        ("Create desktop shortcut", "데스크탑 바로가기 생성"),
        ("Change Path", "경로 변경"),
        ("Create Folder", "폴더 생성"),
        ("Please enter the folder name", "폴더명을 입력해주세요"),
        ("Fix it", "문제 해결"),
        ("Warning", "경고"),
        ("Login screen using Wayland is not supported", "Wayland를 사용한 로그인 화면이 지원되지 않습니다"),
        ("Reboot required", "재부팅이 필요합니다"),
        ("Unsupported display server", "지원하지 않는 디스플레이 서버"),
        ("x11 expected", "x11로 전환해주세요"),
        ("Port", "포트"),
        ("Settings", "설정"),
        ("Username", "사용자명"),
        ("Invalid port", "포트가 유효하지않습니다"),
        ("Closed manually by the peer", "다른 사용자에 의해 종료됨"),
        ("Enable remote configuration modification", "원격 구성 변경 활성화"),
        ("Run without install", "설치 없이 실행"),
        ("Connect via relay", "릴레이를 통해 연결"),
        ("Always connect via relay", "항상 릴레이를 통해 연결"),
        ("whitelist_tip", "화이트리스트에 있는 IP만 나에게 연결할수 있습니다"),
        ("Login", "로그인"),
        ("Verify", "확인"),
        ("Remember me", "기억하기"),
        ("Trust this device", "이 장치 신뢰"),
        ("Verification code", "확인 코드"),
        ("verification_tip", "등록된 이메일 주소로 인증번호가 발송되었습니다. 인증번호를 입력하시면 계속 로그인하실 수 있습니다"),
        ("Logout", "로그아웃"),
        ("Tags", "태그"),
        ("Search ID", "ID 검색"),
        ("whitelist_sep", "다음 글자로 구분합니다. ',(콤마) ;(세미콜론) 띄어쓰기 혹은 줄바꿈'"),
        ("Add ID", "ID 추가"),
        ("Add Tag", "태그 추가"),
        ("Unselect all tags", "모든 태그 선택 해제"),
        ("Network error", "네트워크 오류"),
        ("Username missed", "사용자명이 입력되지않았습니다"),
        ("Password missed", "비밀번호가 입력되지않았습니다"),
        ("Wrong credentials", "로그인 정보가 다릅니다"),
        ("The verification code is incorrect or has expired", "인증 코드가 잘못되었거나 시간이 초과되었습니다."),
        ("Edit Tag", "태그 수정"),
        ("Forget Password", "패스워드 기억하지 않기"),
        ("Favorites", "즐겨찾기"),
        ("Add to Favorites", "즐겨찾기에 추가"),
        ("Remove from Favorites", "즐겨찾기에서 삭제"),
        ("Empty", "비어 있음"),
        ("Invalid folder name", "유효하지 않은 폴더명"),
        ("Socks5 Proxy", "Socks5 프록시"),
        ("Discovered", "찾음"),
        ("install_daemon_tip", "부팅된 이후 시스템 서비스에 설치해야 합니다."),
        ("Remote ID", "원격 ID"),
        ("Paste", "붙여넣기"),
        ("Paste here?", "여기에 붙여넣기를 실핼할까요?"),
        ("Are you sure to close the connection?", "연결을 종료하시겠습니까?"),
        ("Download new version", "최신 버전 다운로드"),
        ("Touch mode", "터치 모드"),
        ("Mouse mode", "마우스 모드"),
        ("One-Finger Tap", "한 손가락 탭"),
        ("Left Mouse", "왼쪽 마우스"),
        ("One-Long Tap", "길게 누르기"),
        ("Two-Finger Tap", "두 손가락 탭"),
        ("Right Mouse", "오른쪽 마우스"),
        ("One-Finger Move", "한 손가락 이동"),
        ("Double Tap & Move", "두 번 탭 하고 이동"),
        ("Mouse Drag", "마우스 드래그"),
        ("Three-Finger vertically", "세 손가락 세로로"),
        ("Mouse Wheel", "마우스 휠"),
        ("Two-Finger Move", "두 손가락 이동"),
        ("Canvas Move", "캔버스 이동"),
        ("Pinch to Zoom", "확대/축소"),
        ("Canvas Zoom", "캔버스 확대"),
        ("Reset canvas", "캔버스 초기화"),
        ("No permission of file transfer", "파일 전송 권한이 없습니다"),
        ("Note", "노트"),
        ("Connection", "연결"),
        ("Share Screen", "화면 공유"),
        ("Chat", "채팅"),
        ("Total", "총합"),
        ("items", "개체"),
        ("Selected", "선택됨"),
        ("Screen Capture", "화면 캡처"),
        ("Input Control", "입력 제어"),
        ("Audio Capture", "오디오 캡처"),
        ("File Connection", "파일 전송"),
        ("Screen Connection", "화면 전송"),
        ("Do you accept?", "동의하십니까?"),
        ("Open System Setting", "시스템 설정 열기"),
        ("How to get Android input permission?", "안드로이드 입력 권한에 어떻게 접근합니까?"),
        ("android_input_permission_tip1", "원격지로서 마우스나 터치를 통해 Android 장치를 제어하려면 RustDesk에서 \"Accessibility (접근성)\" 서비스 사용을 허용해야 합니다."),
        ("android_input_permission_tip2", "시스템 설정 페이지로 이동하여 [설치된 서비스]에서 [RustDesk Input] 서비스를 켜십시오."),
        ("android_new_connection_tip", "현재 장치의 새로운 제어 요청이 수신되었습니다."),
        ("android_service_will_start_tip", "\"화면 캡처\"를 켜면 서비스가 자동으로 시작되어 다른 장치에서 사용자 장치에 대한 연결을 요청할 수 있습니다."),
        ("android_stop_service_tip", "서비스를 종료하면 모든 연결이 자동으로 닫힙니다."),
        ("android_version_audio_tip", "현재 Android 버전은 오디오 캡처를 지원하지 않습니다. Android 10 이상으로 업그레이드하십시오."),
        ("android_start_service_tip", "서비스 시작을 클릭하거나 화면 캡처 권한을 활성화하여 화면 공유 서비스를 시작하세요."),
        ("android_permission_may_not_change_tip", "설정된 연결의 경우 연결이 재설정되지 않는 한 권한이 즉시 변경되지 않을 수 있습니다."),
        ("Account", "계정"),
        ("Overwrite", "덮어쓰기"),
        ("This file exists, skip or overwrite this file?", "해당 파일이 이미 존재합니다, 건너뛰거나 덮어쓰시겠습니까?"),
        ("Quit", "종료"),
        ("Help", "지원"),
        ("Failed", "실패"),
        ("Succeeded", "성공"),
        ("Someone turns on privacy mode, exit", "개인정보 보호 모드가 활성화되어 종료됩니다"),
        ("Unsupported", "지원되지 않음"),
        ("Peer denied", "연결 거부됨"),
        ("Please install plugins", "플러그인을 설치해주세요"),
        ("Peer exit", "다른 사용자가 종료함"),
        ("Failed to turn off", "종료 실패"),
        ("Turned off", "종료됨"),
        ("Language", "언어"),
        ("Keep RustDesk background service", "RustDesk 백그라운드 서비스로 유지하기"),
        ("Ignore Battery Optimizations", "배터리 최적화 무시하기"),
        ("android_open_battery_optimizations_tip", "해당 기능을 비활성화하려면 RustDesk 응용 프로그램 설정 페이지로 이동하여 [배터리]에서 [제한 없음] 선택을 해제하십시오."),
        ("Start on boot", "부팅시 시작"),
        ("Start the screen sharing service on boot, requires special permissions", "부팅 시 화면 공유 서비스를 시작하려면 특별한 권한이 필요합니다."),
        ("Connection not allowed", "연결이 허용되지 않았습니다"),
        ("Legacy mode", "레거시 모드"),
        ("Map mode", "맵 모드"),
        ("Translate mode", "번역 모드"),
        ("Use permanent password", "영구 비밀번호 사용"),
        ("Use both passwords", "(임시/영구) 비밀번호 모두 사용"),
        ("Set permanent password", "영구 비밀번호 설정"),
        ("Enable remote restart", "원격 재시작 활성화"),
        ("Restart remote device", "원격 장치 재시작"),
        ("Are you sure you want to restart", "정말로 재시작 하시겠습니까"),
        ("Restarting remote device", "원격 장치를 재시작하는중"),
        ("remote_restarting_tip", "원격 장치를 재시작하는 중입니다. 이 메시지 상자를 닫고 잠시 후 영구 비밀번호로 다시 연결하십시오."),
        ("Copied", "복사됨"),
        ("Exit Fullscreen", "전체 화면 종료"),
        ("Fullscreen", "전체 화면"),
        ("Mobile Actions", "모바일 액션"),
        ("Select Monitor", "모니터 선택"),
        ("Control Actions", "제어 작업"),
        ("Display Settings", "화면 설정"),
        ("Ratio", "비율"),
        ("Image Quality", "이미지 품질"),
        ("Scroll Style", "스크롤 스타일"),
        ("Show Toolbar", "툴바 보기"),
        ("Hide Toolbar", "툴바 숨기기"),
        ("Direct Connection", "다이렉트 연결"),
        ("Relay Connection", "릴레이 연결"),
        ("Secure Connection", "보안 연결"),
        ("Insecure Connection", "보안되지 않은 연결"),
        ("Scale original", "원래 크기"),
        ("Scale adaptive", "창에 맞게"),
        ("General", "일반"),
        ("Security", "보안"),
        ("Theme", "테마"),
        ("Dark Theme", "다크 테마"),
        ("Light Theme", "라이트 테마"),
        ("Dark", "다크"),
        ("Light", "라이트"),
        ("Follow System", "시스템 설정에따름"),
        ("Enable hardware codec", "하드웨어 코덱 활성화"),
        ("Unlock Security Settings", "보안 설정 잠금 해제"),
        ("Enable audio", "오디오 활성화"),
        ("Unlock Network Settings", "네트워크 설정 잠금 해제"),
        ("Server", "서버"),
        ("Direct IP Access", "다이렉트 IP 연결"),
        ("Proxy", "프록시"),
        ("Apply", "적용"),
        ("Disconnect all devices?", "모든 기기의 연결을 해제하시겠습니까?"),
        ("Clear", "지우기"),
        ("Audio Input Device", "오디오 입력 장치"),
        ("Use IP Whitelisting", "IP 화이트리스트 사용"),
        ("Network", "네트워크"),
        ("Pin Toolbar", "툴바 고정"),
        ("Unpin Toolbar", "툴바 고정 해제"),
        ("Recording", "녹화"),
        ("Directory", "경로"),
        ("Automatically record incoming sessions", "들어오는 세션을 자동으로 녹화"),
        ("Change", "변경"),
        ("Start session recording", "세션 녹화 시작"),
        ("Stop session recording", "세션 녹화 중지"),
        ("Enable recording session", "세션 녹화 활성화"),
        ("Enable LAN discovery", "LAN 검색 활성화"),
        ("Deny LAN discovery", "LAN 검색 거부"),
        ("Write a message", "메시지 쓰기"),
        ("Prompt", "프롬프트"),
        ("Please wait for confirmation of UAC...", "상대방이 UAC를 확인할 때까지 기다려주세요..."),
        ("elevated_foreground_window_tip", "원격 데스크톱의 현재 창을 작동하려면 더 높은 권한이 필요하며 마우스와 키보드를 일시적으로 사용할 수 없는 경우 상대방에게 현재 창을 최소화하도록 요청하거나 연결 관리 창에서 권한 상승을 클릭할 수 있습니다. 이 문제를 방지하려면 원격 장치에 이 소프트웨어를 설치하는 것이 좋습니다"),
        ("Disconnected", "연결이 끊김"),
        ("Other", "기타"),
        ("Confirm before closing multiple tabs", "여러 탭을 닫기 전에 확인"),
        ("Keyboard Settings", "키보드 설정"),
        ("Full Access", "전체 권한"),
        ("Screen Share", "화면 공유"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland는 Ubuntu 21.04 이상 버전이 필요합니다."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland에는 더 높은 버전의 Linux 배포판이 필요합니다. X11 데스크탑을 시도하거나 OS를 변경하십시오."),
        ("JumpLink", "링크연결"),
        ("Please Select the screen to be shared(Operate on the peer side).", "공유할 화면을 선택하십시오(피어 측에서 작동)."),
        ("Show RustDesk", "RustDesk 표시"),
        ("This PC", "이 PC"),
        ("or", "또는"),
        ("Continue with", "계속"),
        ("Elevate", "권한 상승"),
        ("Zoom cursor", "커서 줌"),
        ("Accept sessions via password", "비밀번호를 통해 세션 수락"),
        ("Accept sessions via click", "클릭을 통해 세션 수락"),
        ("Accept sessions via both", "두 가지 모두를 통해 세션을 수락합니다"),
        ("Please wait for the remote side to accept your session request...", "원격 측에서 세션 요청을 수락할 때까지 기다리십시오..."),
        ("One-time Password", "일회용 비밀번호"),
        ("Use one-time password", "일회용 비밀번호 사용"),
        ("One-time password length", "일회용 비밀번호 길이"),
        ("Request access to your device", "접근권한의 허용여부를 요청합니다"),
        ("Hide connection management window", "연결 관리 창 숨기기"),
        ("hide_cm_tip", "숨기기는 비밀번호 연결만 허용되고 고정 비밀번호만 사용되는 경우에만 허용됩니다"),
        ("wayland_experiment_tip", "Wayland 지원은 실험적입니다. 무인 접근이 필요한 경우 X11을 사용하십시오"),
        ("Right click to select tabs", "마우스 오른쪽 버튼을 클릭하고 탭을 선택하세요"),
        ("Skipped", "건너뛰기"),
        ("Add to address book", "주소록에 추가"),
        ("Group", "그룹"),
        ("Search", "검색"),
        ("Closed manually by web console", "웹 콘솔에 의해 수동으로 닫힘"),
        ("Local keyboard type", "로컬 키보드 유형"),
        ("Select local keyboard type", "로컬 키보드 유형 선택"),
        ("software_render_tip", "Nvidia 그래픽 카드를 사용하고 세션이 설정된 후 원격 창이 즉시 닫히는 경우 nouveau 드라이버를 설치하고 소프트웨어 렌더링을 사용하도록 선택하는 것이 도움이 될 수 있습니다. 소프트웨어를 재시작하면 적용됩니다."),
        ("Always use software rendering", "항상 소프트웨어 렌더링 사용"),
        ("config_input", "키보드를 통해 원격 데스크톱을 제어할 수 있으려면 RustDesk의 \"입력 모니터링\" 권한을 부여해 주세요"),
        ("config_microphone", "마이크를 통한 오디오 전송을 지원하려면 RustDesk에 \"녹음\" 권한을 부여해 주세요"),
        ("request_elevation_tip", "상대방이 권한 상승을 요청할 수도 있습니다"),
        ("Wait", "대기"),
        ("Elevation Error", "권한 상승 오류"),
        ("Ask the remote user for authentication", "원격 사용자에게 인증 요청"),
        ("Choose this if the remote account is administrator", "원격 계정이 관리자인 경우 선택하세요"),
        ("Transmit the username and password of administrator", "관리자의 사용자 이름과 비밀번호를 전송합니다"),
        ("still_click_uac_tip", "통제된 사용자는 여전히 RustDesk를 실행하는 UAC 창에서 확인을 클릭해야 합니다"),
        ("Request Elevation", "권한 상승 요청"),
        ("wait_accept_uac_tip", "원격 사용자가 UAC 대화 상자를 확인할 때까지 기다리십시오"),
        ("Elevate successfully", "권한 상승이 완료되었습니다"),
        ("uppercase", "대문자"),
        ("lowercase", "소문자"),
        ("digit", "숫자"),
        ("special character", "특수문자"),
        ("length>=8", "8자 이상"),
        ("Weak", "약함"),
        ("Medium", "보통"),
        ("Strong", "강력"),
        ("Switch Sides", "제어 방향 반전"),
        ("Please confirm if you want to share your desktop?", "데스크탑을 공유하시겠습니까?"),
        ("Display", "디스플레이"),
        ("Default View Style", "기본 보기 스타일"),
        ("Default Scroll Style", "기본 스크롤 스타일"),
        ("Default Image Quality", "기본 이미지 품질"),
        ("Default Codec", "기본 코덱"),
        ("Bitrate", "비트레이트"),
        ("FPS", "FPS"),
        ("Auto", "자동"),
        ("Other Default Options", "기타 기본 옵션"),
        ("Voice call", "음성통화"),
        ("Text chat", "채팅"),
        ("Stop voice call", "음성통화 종료"),
        ("relay_hint_tip", "다이렉트 연결이 안될 수도 있으니 릴레이 연결을 시도해보세요. \n또한 릴레이 연결을 바로 사용하고 싶다면 ID 뒤에 /r을 추가하면 되고, 최근 방문에 해당 카드가 존재한다면 카드 옵션에서 릴레이 연결을 강제하도록 선택할 수도 있습니다."),
        ("Reconnect", "다시 연결"),
        ("Codec", "코덱"),
        ("Resolution", "해상도"),
        ("No transfers in progress", "진행 중인 전송이 없습니다"),
        ("Set one-time password length", "일회용 비밀번호 길이 설정"),
        ("RDP Settings", "RDP 설정"),
        ("Sort by", "정렬 기준"),
        ("New Connection", "새로운 연결"),
        ("Restore", "복원"),
        ("Minimize", "최소화"),
        ("Maximize", "최대화"),
        ("Your Device", "내 장치"),
        ("empty_recent_tip", "최근 세션이 없습니다. 새 세션을 시작해보세요"),
        ("empty_favorite_tip", "장치 즐겨찾기가 없습니다. 새 즐겨찾기를 추가해보세요"),
        ("empty_lan_tip", "제어되는 장치가 발견되지 않았습니다."),
        ("empty_address_book_tip", "현재 주소록에 제어되는 클라이언트가 없습니다"),
        ("eg: admin", "예: 관리자"),
        ("Empty Username", "사용자명이 비어있습니다"),
        ("Empty Password", "비밀번호가 비어있습니다"),
        ("Me", "나"),
        ("identical_file_tip", "이 파일은 상대방의 파일과 일치합니다."),
        ("show_monitors_tip", "도구 모음에 모니터 표시"),
        ("View Mode", "보기 모드"),
        ("login_linux_tip", "X 데스크탑을 활성화하려면 제어되는 터미널의 Linux 계정에 로그인하세요"),
        ("verify_rustdesk_password_tip", "RustDesk 비밀번호 확인"),
        ("remember_account_tip", "이 계정을 기억하세요"),
        ("os_account_desk_tip", "모니터가 없는 환경에서 이 계정은 제어되는 시스템에 로그인하고 데스크탑을 활성화하는 데 사용됩니다"),
        ("OS Account", "OS 계정"),
        ("another_user_login_title_tip", "다른 사용자가 로그인되어 있습니다"),
        ("another_user_login_text_tip", "연결 종료"),
        ("xorg_not_found_title_tip", "Xorg가 설치되지 않았습니다"),
        ("xorg_not_found_text_tip", "Xorg를 설치해주세요"),
        ("no_desktop_title_tip", "데스크탑이 설치되지 않았습니다"),
        ("no_desktop_text_tip", "데스크탑을 설치해주세요"),
        ("No need to elevate", "권한 상승이 필요하지 않습니다."),
        ("System Sound", "시스템 사운드"),
        ("Default", "기본"),
        ("New RDP", "새로운 RDP"),
        ("Fingerprint", "지문"),
        ("Copy Fingerprint", "지문 복사"),
        ("no fingerprints", "지문이 없습니다"),
        ("Select a peer", "동료를 선택하세요"),
        ("Select peers", "동료 선택"),
        ("Plugins", "플러그인"),
        ("Uninstall", "제거"),
        ("Update", "업데이트"),
        ("Enable", "활성화"),
        ("Disable", "비활성화"),
        ("Options", "옵션"),
        ("resolution_original_tip", "기본 해상도"),
        ("resolution_fit_local_tip", "로컬 해상도로 변경"),
        ("resolution_custom_tip", "맞춤 해상도"),
        ("Collapse toolbar", "툴바 접기"),
        ("Accept and Elevate", "권한 상승 승인"),
        ("accept_and_elevate_btn_tooltip", "UAC 권한 상승 및 연결 승인"),
        ("clipboard_wait_response_timeout_tip", "복사 응답 시간이 초과되었습니다."),
        ("Incoming connection", "연결이 요청되었습니다"),
        ("Outgoing connection", "나가는 연결"),
        ("Exit", "나가기"),
        ("Open", "열기"),
        ("logout_tip", "정말 로그아웃하시겠습니까?"),
        ("Service", "서비스"),
        ("Start", "시작"),
        ("Stop", "중지"),
        ("exceed_max_devices", "관리되는 장치가 최대치에 도달했습니다."),
        ("Sync with recent sessions", "최근 세션과 동기화"),
        ("Sort tags", "태그 정렬"),
        ("Open connection in new tab", "새 탭에서 연결 열기"),
        ("Move tab to new window", "탭을 새 창으로 이동"),
        ("Can not be empty", "비워둘 수 없습니다"),
        ("Already exists", "이미 존재 합니다"),
        ("Change Password", "비밀번호 변경"),
        ("Refresh Password", "비밀번호 새로고침"),
        ("ID", "ID"),
        ("Grid View", "그리드 보기"),
        ("List View", "리스트 보기"),
        ("Select", "선택"),
        ("Toggle Tags", "태그 전환"),
        ("pull_ab_failed_tip", "주소록을 가져오지 못했습니다."),
        ("push_ab_failed_tip", "주소록 업로드 실패"),
        ("synced_peer_readded_tip", "최근 세션에 있는 장치는 주소록에 다시 동기화됩니다."),
        ("Change Color", "색상 변경"),
        ("Primary Color", "기본 색상"),
        ("HSV Color", "HSV 색상"),
        ("Installation Successful!", "설치 성공!"),
        ("Installation failed!", "설치 실패!"),
        ("Reverse mouse wheel", "마우스 휠 반전"),
        ("{} sessions", "{} 세션"),
        ("scam_title", "당신은 사기를 당했을 수도 있습니다"),
        ("scam_text1", "모르는 사람과 통화 중이고 그들이 RustDesk를 사용하여 서비스를 시작하라고 요청하는 경우, 계속하지 말고 즉시 전화를 끊으세요"),
        ("scam_text2", "그들은 귀하의 돈이나 기타 개인 정보를 훔치려는 사기꾼일 가능성이 높습니다"),
        ("Don't show again", "다시 표시하지 않음"),
        ("I Agree", "동의"),
        ("Decline", "거절"),
        ("Timeout in minutes", "시간 초과(분)"),
        ("auto_disconnect_option_tip", "비활성 세션 자동 종료"),
        ("Connection failed due to inactivity", "장시간 활동이 없어 연결이 자동으로 종료되었습니다"),
        ("Check for software update on startup", "시작 시 소프트웨어 업데이트 확인"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "RustDesk Server Pro를 {} 버전 이상으로 업그레이드하십시오!"),
        ("pull_group_failed_tip", "그룹 정보를 가져오지 못했습니다"),
        ("Filter by intersection", "교차로로 필터링"),
        ("Remove wallpaper during incoming sessions", "세션 수락시 배경화면 제거"),
        ("Test", "테스트"),
        ("display_is_plugged_out_msg", "디스플레이가 연결되어 있지 않습니다. 첫 번째 디스플레이로 전환하세요."),
        ("No displays", "디스플레이 없음"),
        ("elevated_switch_display_msg", "권한 상승된 사용자 모드에서는 다중 디스플레이가 지원되지 않으므로 기본 디스플레이로 전환하세요."),
        ("Open in new window", "새 창에서 열기"),
        ("Show displays as individual windows", "디스플레이를 개별 창으로 표시"),
        ("Use all my displays for the remote session", "원격 세션에 내 디스플레이를 모두 사용"),
        ("selinux_tip", "SELinux를 활성화하면 RustDesk가 호스트로 제대로 실행되지 않을 수 있습니다"),
        ("Change view", "보기 변경"),
        ("Big tiles", "큰 타일"),
        ("Small tiles", "작은 타일"),
        ("List", "리스트"),
        ("Virtual display", "가상 디스플레이"),
        ("Plug out all", "전원 모두 끄기"),
        ("True color (4:4:4)", "트루컬러 (4:4:4)"),
        ("Enable blocking user input", "사용자 입력 차단 허용"),
        ("id_input_tip", "입력된 ID, IP, 도메인과 포트(<domain>:<port>)를 입력할 수 있습니다.\n다른 서버에 있는 장치에 연결하려면 서버 주소(<id>@<server_address>?key=<key_value>)를 추가하세요"),
        ("privacy_mode_impl_mag_tip", "모드 1"),
        ("privacy_mode_impl_virtual_display_tip", "모드 2"),
        ("Enter privacy mode", "개인정보 보호 모드 사용"),
        ("Exit privacy mode", "개인정보 보호 모드 종료"),
        ("idd_not_support_under_win10_2004_tip", "간접 디스플레이 드라이버는 지원되지 않습니다. Windows 10 버전 2004 이상이 필요합니다."),
        ("switch_display_elevated_connections_tip", "여러 연결이 있는 경우 권한 상승된 사용자 모드에서는 기본이 아닌 디스플레이로 전환이 지원되지 않습니다. 다중 디스플레이를 제어하려면 설치 후 재시도하세요."),
        ("input_source_1_tip", "입력소스 1"),
        ("input_source_2_tip", "입력소스 2"),
        ("capture_display_elevated_connections_tip", "권한 상승된 사용자 모드에서는 다중 디스플레이 캡처가 지원되지 않습니다. 다중 디스플레이를 제어하려면 설치 후 재시도하세요."),
        ("Swap control-command key", "Control 및 Command 키 교체"),
        ("swap-left-right-mouse", "마우스 왼쪽 버튼과 오른쪽 버튼 바꾸기"),
        ("2FA code", "2단계 인증 코드"),
        ("More", "더보기"),
        ("enable-2fa-title", "2단계 인증 활성화"),
        ("enable-2fa-desc", "지금 인증자를 설정하세요. 휴대폰이나 데스크톱 컴퓨터에서 Authy, Microsoft 또는 Google Authenticator와 같은 인증기를 사용할 수 있습니다. 인증기로 QR 코드를 스캔하고 표시된 코드를 입력하면 2단계 인증이 활성화됩니다. "),
        ("wrong-2fa-code", "이 코드는 확인할 수 없습니다. 인증 코드와 현지 시간 설정이 올바른지 확인하세요."),
        ("enter-2fa-title", "2단계 인증"),
        ("Email verification code must be 6 characters.", "이메일 인증 코드는 6자여야 합니다."),
        ("2FA code must be 6 digits.", "2단계 인증 코드는 6자리여야 합니다."),
        ("Multiple Windows sessions found", "여러 Windows 세션이 발견되었습니다."),
        ("Please select the session you want to connect to", "연결하려는 세션을 선택하세요."),
        ("powered_by_me", "RustDesk 제공"),
        ("outgoing_only_desk_tip", "이것은 맞춤형 버전입니다.\n다른 장치에 연결할 수 있지만 다른 장치는 귀하의 장치에 연결할 수 없습니다."),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
    ].iter().cloned().collect();
}
