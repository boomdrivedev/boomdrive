// The Boomdrive launcher's menu on macOS (record 0023 as amended on
// 2026-11-11; design Decision 3 of the change
// each-drive-opens-from-the-launcher): Boomdrive.app's executable, a
// status item titled Boomdrive in the menu bar. Its menu holds an item for
// each drive of the bundle's Info.plist, BoomdriveDrives, titled the
// drive's name, then Default stream, then Quit Boomdrive. Picking a
// drive's item runs the bundle's script, boomdrive-launcher, with the
// drive's session as its one argument, and Default stream runs it with
// none; the script is not waited on, so the menu answers at once, and it
// shows Flume's refusal itself. Where the script cannot be started, an
// alert titled Boomdrive gives the error's own words.
//
// It runs from the person's opening it until they quit it or log out: no
// module or service starts it (core.13, core.46). It runs Flume's command
// through the script alone, and names no multiplexer (core.29).
#import <AppKit/AppKit.h>

@interface BoomdriveMenu : NSObject <NSApplicationDelegate>
@property(strong) NSStatusItem *statusItem;
@end

@implementation BoomdriveMenu

- (void)applicationDidFinishLaunching:(NSNotification *)notification {
  NSMenu *menu = [[NSMenu alloc] initWithTitle:@"Boomdrive"];
  id drives =
      [[NSBundle mainBundle] objectForInfoDictionaryKey:@"BoomdriveDrives"];
  if ([drives isKindOfClass:[NSArray class]]) {
    for (id entry in drives) {
      // A dictionary with no string drive or session is left out.
      if (![entry isKindOfClass:[NSDictionary class]]) {
        continue;
      }
      id drive = entry[@"drive"];
      id session = entry[@"session"];
      if (![drive isKindOfClass:[NSString class]] ||
          ![session isKindOfClass:[NSString class]]) {
        continue;
      }
      NSMenuItem *item = [[NSMenuItem alloc] initWithTitle:drive
                                                    action:@selector(open:)
                                             keyEquivalent:@""];
      item.target = self;
      item.representedObject = session;
      [menu addItem:item];
    }
  }

  NSMenuItem *fallback = [[NSMenuItem alloc] initWithTitle:@"Default stream"
                                                    action:@selector(open:)
                                             keyEquivalent:@""];
  fallback.target = self;
  fallback.representedObject = nil;
  [menu addItem:fallback];

  [menu addItem:[NSMenuItem separatorItem]];
  [menu addItemWithTitle:@"Quit Boomdrive"
                  action:@selector(terminate:)
           keyEquivalent:@"q"];

  self.statusItem = [[NSStatusBar systemStatusBar]
      statusItemWithLength:NSVariableStatusItemLength];
  self.statusItem.button.title = @"Boomdrive";
  self.statusItem.menu = menu;
}

// Runs the bundle's script with the item's session, or with none for
// Default stream, and does not wait on it.
- (void)open:(NSMenuItem *)sender {
  NSString *session = sender.representedObject;
  NSArray<NSString *> *arguments = session ? @[ session ] : @[];
  NSURL *script =
      [[NSBundle mainBundle] URLForAuxiliaryExecutable:@"boomdrive-launcher"];
  NSError *error = nil;
  NSTask *task = nil;
  if (script == nil) {
    error = [NSError errorWithDomain:NSCocoaErrorDomain
                                code:NSFileNoSuchFileError
                            userInfo:@{
                              NSLocalizedDescriptionKey :
                                  @"The bundle holds no boomdrive-launcher."
                            }];
  } else {
    task = [NSTask launchedTaskWithExecutableURL:script
                                       arguments:arguments
                                           error:&error
                              terminationHandler:nil];
  }
  if (task == nil) {
    NSAlert *alert = [[NSAlert alloc] init];
    alert.messageText = @"Boomdrive";
    alert.informativeText = error.localizedDescription;
    [alert runModal];
  }
}

@end

// The application's delegate, which it holds weakly, kept for as long as
// the app runs.
static BoomdriveMenu *delegate;

int main(void) {
  @autoreleasepool {
    NSApplication *application = [NSApplication sharedApplication];
    [application setActivationPolicy:NSApplicationActivationPolicyAccessory];
    delegate = [[BoomdriveMenu alloc] init];
    application.delegate = delegate;
    [application run];
  }
  return 0;
}
